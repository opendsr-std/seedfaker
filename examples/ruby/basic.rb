#!/usr/bin/env ruby
# Install:  gem install seedfaker
# Requires: libseedfaker_ffi shared library
# Docs:     https://github.com/opendsr-std/seedfaker

require "seedfaker"

f = Seedfaker::SeedFaker.new(seed: "demo", locale: "en")

puts f.field("name")
puts f.field("email")
puts f.field("phone")

# ctx=strict → name, email, phone correlated per row
f.records(%w[name email phone], n: 5, ctx: "strict").each do |r|
  printf "%s\t%s\t%s\n", r["name"], r["email"], r["phone"]
end

a = Seedfaker::SeedFaker.new(seed: "ci")
b = Seedfaker::SeedFaker.new(seed: "ci")
raise "determinism failed" unless a.field("name") == b.field("name")

puts "fields: #{Seedfaker::SeedFaker.fields.size}"
puts "fingerprint: #{Seedfaker::SeedFaker.fingerprint}"
