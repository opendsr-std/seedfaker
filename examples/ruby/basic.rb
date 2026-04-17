#!/usr/bin/env ruby
# seedfaker — deterministic synthetic data generator
#
# Install:  gem install seedfaker
# Requires: libseedfaker_ffi shared library
# Docs:     https://github.com/opendsr-std/seedfaker

require "seedfaker"

# Deterministic: same seed = same output, always
f = Seedfaker::SeedFaker.new(seed: "demo", locale: "en")

puts f.field("name")
puts f.field("email")
puts f.field("phone")

# Correlated records: email derived from name, phone matches locale
records = f.records(%w[name email phone], n: 5, ctx: "strict")
records.each do |r|
  printf "%s\t%s\t%s\n", r["name"], r["email"], r["phone"]
end

# Verify determinism
a = Seedfaker::SeedFaker.new(seed: "ci")
b = Seedfaker::SeedFaker.new(seed: "ci")
raise "determinism failed" unless a.field("name") == b.field("name")

puts "fields: #{Seedfaker::SeedFaker.fields.size}"
puts "fingerprint: #{Seedfaker::SeedFaker.fingerprint}"
