Gem::Specification.new do |s|
  s.name        = "seedfaker"
  s.version     = "0.3.0.pre.alpha.2"
  s.summary     = "Deterministic synthetic data generator"
  s.description = "214 fields, 68 locales. Same seed = same output. Native FFI + CLI fallback."
  s.authors     = ["Eduard Titov"]
  s.email       = "editied@gmail.com"
  s.homepage    = "https://github.com/opendsr-std/seedfaker"
  s.license     = "MIT"
  s.files       = Dir["lib/**/*.rb", "bin/**/*", "seedfaker.gemspec"]
  s.required_ruby_version = ">= 2.7"
end
