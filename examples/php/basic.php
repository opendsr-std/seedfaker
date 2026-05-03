<?php
/**
 * Install:  composer require opendsr/seedfaker
 * Requires: PHP >= 8.1, FFI extension, libseedfaker_ffi shared library
 * Docs:     https://github.com/opendsr-std/seedfaker
 */

require_once __DIR__ . "/../../packages/php/src/SeedFaker.php";

use Seedfaker\SeedFaker;

$f = new SeedFaker(seed: "demo", locale: "en");

echo $f->field("name") . "\n";
echo $f->field("email") . "\n";
echo $f->field("phone") . "\n";

// ctx=strict → name, email, phone correlated per row
foreach ($f->records(["name", "email", "phone"], n: 5, ctx: "strict") as $r) {
    printf("%s\t%s\t%s\n", $r["name"], $r["email"], $r["phone"]);
}

$a = new SeedFaker(seed: "ci");
$b = new SeedFaker(seed: "ci");
if ($a->field("name") !== $b->field("name")) {
    throw new RuntimeException("determinism failed");
}

echo "fields: " . count(SeedFaker::fields()) . "\n";
echo "fingerprint: " . SeedFaker::fingerprint() . "\n";
