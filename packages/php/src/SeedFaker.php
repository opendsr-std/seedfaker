<?php

declare(strict_types=1);

namespace Seedfaker;

class SeedFaker
{
    private const CDEF = <<<'CDEF'
    typedef struct SfFaker SfFaker;
    SfFaker* sf_create(const char* opts_json);
    void sf_destroy(SfFaker* faker);
    char* sf_field(SfFaker* faker, const char* field_spec);
    char* sf_validate(const SfFaker* faker, const char* opts_json);
    char* sf_record(SfFaker* faker, const char* opts_json);
    char* sf_records(SfFaker* faker, const char* opts_json);
    char* sf_fields_json(void);
    char* sf_fingerprint(void);
    char* sf_build_info(void);
    void sf_free(char* ptr);
    const char* sf_last_error(void);
    CDEF;

    private \FFI $ffi;
    private \FFI\CData $handle;

    public function __construct(
        ?string $seed = null,
        ?string $locale = null,
        ?string $tz = null,
        ?int $since = null,
        ?int $until = null
    ) {
        $lib = self::findLibrary();
        if ($lib === null || !extension_loaded('ffi')) {
            throw new \RuntimeException(
                'seedfaker: native FFI library not found or FFI extension not loaded'
            );
        }

        $this->ffi = \FFI::cdef(self::CDEF, $lib);
        $opts = json_encode(array_filter([
            'seed' => $seed,
            'locale' => $locale,
            'tz' => $tz,
            'since' => $since,
            'until' => $until,
        ], fn($v) => $v !== null), JSON_THROW_ON_ERROR);
        $this->handle = $this->ffi->sf_create($opts);
        if ($this->handle === null) {
            $err = $this->ffi->sf_last_error();
            throw new \RuntimeException("sf_create failed: $err");
        }
    }

    public function __destruct()
    {
        $this->ffi->sf_destroy($this->handle);
    }

    /** @return ($n is 1 ? string : string[]) */
    // @field-params-start
    public function field(
        string $name,
        int $n = 1,
        bool $sign = false,
        bool $byte = false,
        bool $hex = false,
        bool $rgb = false,
        bool $rgba = false,
        bool $plain = false,
        bool $unix = false,
        bool $ms = false,
        bool $log = false,
        bool $us = false,
        bool $eu = false,
        bool $xuniq = false,
        bool $e164 = false,
        bool $intl = false,
        bool $alpha3 = false,
        bool $numeric = false,
        bool $short = false,
        bool $underscore = false,
        bool $space = false,
        bool $dash = false,
        bool $dot = false,
        bool $comma = false,
        bool $usd = false,
        bool $eur = false,
        bool $gbp = false,
        bool $crypto = false,
        bool $pin = false,
        bool $memorable = false,
        bool $mixed = false,
        bool $strong = false,
        bool $international = false,
        bool $internal = false,
        bool $http = false,
        bool $https = false,
        bool $ftp = false,
        bool $ws = false,
        bool $wss = false,
        bool $ssh = false,
        bool $system = false,
        bool $registered = false,
        bool $dynamic = false,
        bool $unprivileged = false,
        bool $service = false,
        bool $fast = false,
        bool $slow = false,
        bool $seconds = false,
        bool $r1x1 = false,
        bool $r4x3 = false,
        bool $r3x2 = false,
        bool $r16x9 = false,
        bool $r21x9 = false,
        bool $r9x16 = false,
        bool $r3x4 = false,
        bool $r2x3 = false,
        bool $btc = false,
        bool $multi = false,
        bool $upper = false,
        bool $lower = false,
        bool $capitalize = false,
        bool $asc = false,
        bool $desc = false,
        ?int $omit = null,
        ?int $length = null,
        ?array $range = null,
    ): string|array {
    // @field-params-end
        $spec = self::buildSpec($name, get_defined_vars());
        if ($n === 1) {
            return $this->fieldOne($spec);
        }
        $vals = [];
        for ($i = 0; $i < $n; $i++) {
            $vals[] = $this->fieldOne($spec);
        }
        return $vals;
    }

    private function fieldOne(string $spec): string
    {
        $ptr = $this->ffi->sf_field($this->handle, $spec);
        if ($ptr === null) {
            $err = $this->ffi->sf_last_error();
            throw new \RuntimeException("sf_field failed: $err");
        }
        $val = \FFI::string($ptr);
        $this->ffi->sf_free($ptr);
        return $val;
    }

    /**
     * @param string[] $fields
     * @return array<string, string>
     */
    public function record(
        array $fields,
        ?string $ctx = null,
        ?string $corrupt = null
    ): array {
        $opts = json_encode(array_filter([
            'fields' => $fields,
            'ctx' => $ctx,
            'corrupt' => $corrupt,
        ], fn($v) => $v !== null), JSON_THROW_ON_ERROR);
        $ptr = $this->ffi->sf_record($this->handle, $opts);
        if ($ptr === null) {
            $err = $this->ffi->sf_last_error();
            throw new \RuntimeException("sf_record failed: $err");
        }
        $json = \FFI::string($ptr);
        $this->ffi->sf_free($ptr);
        return json_decode($json, true, 512, JSON_THROW_ON_ERROR);
    }

    /**
     * @param string[] $fields
     * @return array<int, array<string, string>>
     */
    public function records(
        array $fields,
        int $n = 1,
        ?string $ctx = null,
        ?string $corrupt = null
    ): array {
        $opts = json_encode(array_filter([
            'fields' => $fields,
            'n' => $n,
            'ctx' => $ctx,
            'corrupt' => $corrupt,
        ], fn($v) => $v !== null), JSON_THROW_ON_ERROR);
        $ptr = $this->ffi->sf_records($this->handle, $opts);
        if ($ptr === null) {
            $err = $this->ffi->sf_last_error();
            throw new \RuntimeException("sf_records failed: $err");
        }
        $json = \FFI::string($ptr);
        $this->ffi->sf_free($ptr);
        return json_decode($json, true, 512, JSON_THROW_ON_ERROR);
    }

    /**
     * @param string[] $fields
     */
    public function validate(
        array $fields,
        ?string $ctx = null,
        ?string $corrupt = null
    ): void {
        $opts = json_encode(array_filter([
            'fields' => $fields,
            'ctx' => $ctx,
            'corrupt' => $corrupt,
        ], fn($v) => $v !== null), JSON_THROW_ON_ERROR);
        $ptr = $this->ffi->sf_validate($this->handle, $opts);
        if ($ptr === null) {
            $err = $this->ffi->sf_last_error();
            throw new \RuntimeException("validation failed: $err");
        }
        $this->ffi->sf_free($ptr);
    }

    /** @return string[] */
    public static function fields(): array
    {
        $lib = self::findLibrary();
        if ($lib === null || !extension_loaded('ffi')) {
            throw new \RuntimeException('seedfaker: native FFI library not found');
        }

        $ffi = \FFI::cdef(self::CDEF, $lib);
        $ptr = $ffi->sf_fields_json();
        $json = \FFI::string($ptr);
        $ffi->sf_free($ptr);
        return array_column(json_decode($json, true), 'name');
    }

    public static function fingerprint(): string
    {
        $lib = self::findLibrary();
        if ($lib === null || !extension_loaded('ffi')) {
            throw new \RuntimeException('seedfaker: native FFI library not found');
        }

        $ffi = \FFI::cdef(self::CDEF, $lib);
        $ptr = $ffi->sf_fingerprint();
        $val = \FFI::string($ptr);
        $ffi->sf_free($ptr);
        return $val;
    }

    private static function buildSpec(string $name, array $vars): string
    {
        $skip = ['name', 'n', 'this'];
        $parts = [$name];
        foreach ($vars as $k => $v) {
            if (in_array($k, $skip, true)) {
                continue;
            }
            // Strip 'r' prefix added for digit-starting modifiers (r1x1 → 1x1)
            $seg = (strlen($k) > 1 && $k[0] === 'r' && ctype_digit($k[1])) ? substr($k, 1) : $k;
            if ($k === 'range' && is_array($v)) {
                $parts[] = $v[0] . '..' . $v[1];
            } elseif ($v === true) {
                $parts[] = $seg;
            } elseif (is_int($v) && $v > 0 && $k === 'omit') {
                $parts[] = "omit=$v";
            } elseif (is_int($v) && $v > 0 && $k === 'length') {
                $parts[] = (string)$v;
            }
        }
        return implode(':', $parts);
    }

    // CI replaces entries between markers with real SHA256 values before publishing.
    // Empty array = dev (running from source). Populated = production (verify mandatory).
    private const NATIVE_CHECKSUMS = [
        // @checksums-start
        // @checksums-end
    ];

    private static function detectPlatform(): string
    {
        $os = match (PHP_OS_FAMILY) {
            'Darwin' => 'darwin',
            'Linux' => 'linux',
            'Windows' => 'windows',
            default => throw new \RuntimeException(
                'seedfaker: unsupported OS family ' . PHP_OS_FAMILY
            ),
        };
        $machine = strtolower(php_uname('m'));
        $arch = match (true) {
            in_array($machine, ['x86_64', 'amd64'], true) => 'x86_64',
            in_array($machine, ['arm64', 'aarch64'], true) => 'arm64',
            default => throw new \RuntimeException(
                "seedfaker: unsupported architecture $machine"
            ),
        };
        return "$os-$arch";
    }

    private static function libraryName(): string
    {
        $ext = match (PHP_OS_FAMILY) {
            'Windows' => 'dll',
            'Darwin' => 'dylib',
            default => 'so',
        };
        return "libseedfaker_ffi.$ext";
    }

    private static function findLibrary(): ?string
    {
        $dir = dirname(__DIR__);
        $name = self::libraryName();
        $platform = self::detectPlatform();

        if (!empty(self::NATIVE_CHECKSUMS)) {
            // Production: bundled per-platform binary, verify SHA256, no fallback.
            $bundled = "$dir/bin/$platform/$name";
            if (!file_exists($bundled)) {
                throw new \RuntimeException(
                    "seedfaker: bundled library for $platform not found at $bundled"
                );
            }
            $expected = self::NATIVE_CHECKSUMS[$platform] ?? null;
            if ($expected === null) {
                throw new \RuntimeException(
                    "seedfaker: no checksum registered for platform $platform"
                );
            }
            $actual = hash_file('sha256', $bundled);
            if ($actual !== $expected) {
                throw new \RuntimeException(
                    "seedfaker: native library integrity check failed for $platform. " .
                    "Expected " . substr($expected, 0, 16) . "..., " .
                    "got " . substr($actual, 0, 16) . "... " .
                    "Reinstall the package."
                );
            }
            return $bundled;
        }

        // Dev: NATIVE_CHECKSUMS empty = running from source.
        $project = dirname($dir, 2) . "/rust/target/release/$name";
        if (file_exists($project)) return $project;

        return null;
    }
}
