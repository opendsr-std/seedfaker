# frozen_string_literal: true

require "digest"
require "fiddle"
require "fiddle/import"
require "json"

module Seedfaker
  class SeedFaker
    def initialize(seed: nil, locale: nil, tz: nil, since: nil, until_time: nil)
      lib = Seedfaker.find_library
      raise "seedfaker: native FFI library not found" unless lib

      @ffi = Fiddle.dlopen(lib)
      opts = JSON.generate({
        seed: seed, locale: locale, tz: tz,
        since: since&.to_i, until: until_time&.to_i
      }.compact)
      @sf_create = Fiddle::Function.new(@ffi["sf_create"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
      @sf_destroy = Fiddle::Function.new(@ffi["sf_destroy"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)
      @sf_field = Fiddle::Function.new(@ffi["sf_field"], [Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
      @sf_records = Fiddle::Function.new(@ffi["sf_records"], [Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
      @sf_record = Fiddle::Function.new(@ffi["sf_record"], [Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
      @sf_validate = Fiddle::Function.new(@ffi["sf_validate"], [Fiddle::TYPE_VOIDP, Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
      @sf_free = Fiddle::Function.new(@ffi["sf_free"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)
      @sf_last_error = Fiddle::Function.new(@ffi["sf_last_error"], [], Fiddle::TYPE_VOIDP)
      @handle = @sf_create.call(opts)
      raise "sf_create failed: #{last_error}" if @handle.null?

      ObjectSpace.define_finalizer(self, SeedFaker.invoke_release(@sf_destroy, @handle))
    end

    def self.invoke_release(sf_destroy, handle)
      proc { sf_destroy.call(handle) }
    end

    def field(name, n: 1, **opts)
      spec = build_spec(name, opts)
      if n == 1
        field_one(spec)
      else
        Array.new(n) { field_one(spec) }
      end
    end

    def record(fields, ctx: nil, corrupt: nil)
      opts = JSON.generate({ fields: fields, ctx: ctx, corrupt: corrupt }.compact)
      ptr = @sf_record.call(@handle, opts)
      raise "sf_record failed: #{last_error}" if ptr.null?
      json = ptr.to_s
      @sf_free.call(ptr)
      JSON.parse(json)
    end

    def records(fields, n: 1, ctx: nil, corrupt: nil)
      opts = JSON.generate({ fields: fields, n: n, ctx: ctx, corrupt: corrupt }.compact)
      ptr = @sf_records.call(@handle, opts)
      raise "sf_records failed: #{last_error}" if ptr.null?
      json = ptr.to_s
      @sf_free.call(ptr)
      JSON.parse(json)
    end

    def validate(fields, ctx: nil, corrupt: nil)
      opts = JSON.generate({ fields: fields, ctx: ctx, corrupt: corrupt }.compact)
      ptr = @sf_validate.call(@handle, opts)
      raise "validate failed: #{last_error}" if ptr.null?
      @sf_free.call(ptr)
      nil
    end

    def self.fields
      lib = Seedfaker.find_library
      raise "seedfaker: native FFI library not found" unless lib

      ffi = Fiddle.dlopen(lib)
      sf_fields = Fiddle::Function.new(ffi["sf_fields_json"], [], Fiddle::TYPE_VOIDP)
      sf_free = Fiddle::Function.new(ffi["sf_free"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)
      ptr = sf_fields.call
      json = ptr.to_s
      sf_free.call(ptr)
      JSON.parse(json).map { |f| f["name"] }
    end

    def self.fingerprint
      lib = Seedfaker.find_library
      raise "seedfaker: native FFI library not found" unless lib

      ffi = Fiddle.dlopen(lib)
      sf_fp = Fiddle::Function.new(ffi["sf_fingerprint"], [], Fiddle::TYPE_VOIDP)
      sf_free = Fiddle::Function.new(ffi["sf_free"], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)
      ptr = sf_fp.call
      val = ptr.to_s
      sf_free.call(ptr)
      val
    end

    private

    def field_one(spec)
      ptr = @sf_field.call(@handle, spec)
      raise "sf_field failed for #{spec}: #{last_error}" if ptr.null?
      val = ptr.to_s
      @sf_free.call(ptr)
      val
    end

    def build_spec(name, opts)
      skip = %i[n]
      parts = [name.to_s]
      opts.each do |k, v|
        next if skip.include?(k)
        # Strip 'r' prefix for digit-starting modifiers (r1x1 → 1x1)
        seg = k.to_s
        seg = seg[1..] if seg.length > 1 && seg[0] == "r" && seg[1].match?(/\d/)
        if k == :range && v.is_a?(Array)
          parts << "#{v[0]}..#{v[1]}"
        elsif v == true
          parts << seg
        elsif v.is_a?(Integer) && k == :omit
          parts << "omit=#{v}"
        elsif v.is_a?(Integer) && k == :length
          parts << v.to_s
        end
      end
      parts.join(":")
    end

    def last_error
      ptr = @sf_last_error.call
      ptr.null? ? "unknown error" : ptr.to_s
    end

  end

  # @checksum-start
  # CI replaces this with real SHA256 before building gem.
  # Empty = dev (running from source). Filled = production (verify mandatory).
  NATIVE_CHECKSUM = ""
  # @checksum-end

  def self.find_library
    os = case RbConfig::CONFIG["host_os"]
         when /darwin/ then "darwin"
         when /linux/ then "linux"
         when /mswin|mingw|cygwin/ then "windows"
         else RbConfig::CONFIG["host_os"]
         end
    cpu = RbConfig::CONFIG["host_cpu"]
    arch = if cpu.match?(/x86_64|amd64/) then "x86_64"
           elsif cpu.match?(/arm64|aarch64/) then "arm64"
           else cpu
           end
    ext = case os
          when "darwin" then "dylib"
          when "windows" then "dll"
          else "so"
          end
    name = "libseedfaker_ffi.#{ext}"
    target = "#{os}-#{arch}"

    # Gem layout: bin/{os}-{arch}/libseedfaker_ffi.{ext}
    bundled = File.join(__dir__, "..", "bin", target, name)

    if NATIVE_CHECKSUM != ""
      # Production: bundled binary only, verify SHA256, no exceptions.
      raise "seedfaker: bundled library not found at #{bundled}" unless File.exist?(bundled)
      actual = Digest::SHA256.hexdigest(File.binread(bundled))
      unless actual == NATIVE_CHECKSUM
        raise "seedfaker: native library integrity check failed. " \
              "Expected #{NATIVE_CHECKSUM[0, 16]}..., got #{actual[0, 16]}... " \
              "Reinstall the package or verify your installation."
      end
      return bundled
    end

    # Dev: NATIVE_CHECKSUM empty = running from source or flat-layout gem.
    return bundled if File.exist?(bundled)

    project = File.join(__dir__, "..", "..", "..", "rust", "target", "release", name)
    return project if File.exist?(project)

    nil
  end
end
