<?php
/**
 * seedfaker — deterministic synthetic data generator
 *
 * Install:  composer require opendsr/seedfaker
 * Requires: PHP >= 7.4, FFI extension, libseedfaker_ffi shared library
 * Docs:     https://github.com/opendsr-std/seedfaker
 */

require_once __DIR__ . "/../../packages/php/src/SeedFaker.php";

use Seedfaker\SeedFaker;

// Deterministic: same seed = same output, always
$f = new SeedFaker(seed: "demo", locale: "en");

echo $f->field("name") . "\n";
echo $f->field("email") . "\n";
echo $f->field("phone") . "\n";

// Correlated records: email derived from name, phone matches locale
$records = $f->records(["name", "email", "phone"], n: 5, ctx: "strict");
foreach ($records as $r) {
    printf("%s\t%s\t%s\n", $r["name"], $r["email"], $r["phone"]);
}

// Verify determinism
$a = new SeedFaker(seed: "ci");
$b = new SeedFaker(seed: "ci");
assert($a->field("name") === $b->field("name"));

echo "fields: " . count(SeedFaker::fields()) . "\n";
echo "fingerprint: " . SeedFaker::fingerprint() . "\n";
