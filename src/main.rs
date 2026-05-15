// ============================================================
//  Sistem Monitoring Kelembaban Industri
//  Mata Kuliah : Algoritma dan Pemrograman
//  Bahasa      : Rust
//  Deskripsi   : Console application monitoring kelembaban
//                dengan OOP (struct + impl), komputasi numerik,
//                alarm otomatis, dan kontrol humidifier.
// ============================================================

use std::collections::VecDeque;
use std::io::{self, Write};

// ============================================================
// KONSTANTA SISTEM
// ============================================================
const KELEMBABAN_MIN: f32 = 30.0;   // % RH
const KELEMBABAN_MAX: f32 = 80.0;   // % RH
const WINDOW_MA: usize   = 5;       // jumlah data moving average
const SENSOR_MIN_VALID: f32 = 0.0;
const SENSOR_MAX_VALID: f32 = 100.0;

// ============================================================
// STRUCT: Sensor
// Merepresentasikan sensor kelembaban fisik
// ============================================================
struct Sensor {
    name      : String,
    value     : f32,
    is_valid  : bool,
    offset    : f32,   // kalibrasi offset
    gain      : f32,   // kalibrasi gain
}

impl Sensor {
    /// Buat sensor baru dengan nilai kalibrasi default
    fn new(name: &str) -> Sensor {
        Sensor {
            name    : name.to_string(),
            value   : 0.0,
            is_valid: false,
            offset  : 0.0,
            gain    : 1.0,
        }
    }

    /// Set nilai kalibrasi sensor
    fn set_kalibrasi(&mut self, offset: f32, gain: f32) {
        self.offset = offset;
        self.gain   = gain;
    }

    /// Baca nilai mentah dan terapkan kalibrasi
    fn baca(&mut self, raw_value: f32) {
        if raw_value >= SENSOR_MIN_VALID && raw_value <= SENSOR_MAX_VALID {
            // Rumus kalibrasi: y = gain * x + offset
            self.value    = self.gain * raw_value + self.offset;
            self.value    = self.value.clamp(0.0, 100.0);
            self.is_valid = true;
        } else {
            self.value    = 0.0;
            self.is_valid = false;
        }
    }

    /// Hitung error pengukuran (selisih dari nilai referensi)
    fn hitung_error(&self, referensi: f32) -> f32 {
        (self.value - referensi).abs()
    }

    fn display(&self) {
        println!(
            "  [SENSOR] {} | Nilai: {:.2}% RH | Status: {}",
            self.name,
            self.value,
            if self.is_valid { "OK" } else { "ERROR" }
        );
    }
}

// ============================================================
// STRUCT: MonitoringSystem
// Menyimpan riwayat data dan menghitung statistik
// ============================================================
struct MonitoringSystem {
    nama_sistem : String,
    riwayat     : VecDeque<f32>,   // buffer moving average
    total_baca  : u32,
    total_alarm : u32,
}

impl MonitoringSystem {
    fn new(nama: &str) -> MonitoringSystem {
        MonitoringSystem {
            nama_sistem : nama.to_string(),
            riwayat     : VecDeque::with_capacity(WINDOW_MA),
            total_baca  : 0,
            total_alarm : 0,
        }
    }

    /// Tambah data baru ke buffer
    fn tambah_data(&mut self, nilai: f32) {
        if self.riwayat.len() == WINDOW_MA {
            self.riwayat.pop_front();
        }
        self.riwayat.push_back(nilai);
        self.total_baca += 1;
    }

    /// Hitung Moving Average dari buffer
    fn moving_average(&self) -> f32 {
        if self.riwayat.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.riwayat.iter().sum();
        sum / self.riwayat.len() as f32
    }

    /// Hitung simpangan (standar deviasi sederhana)
    fn simpangan(&self) -> f32 {
        if self.riwayat.len() < 2 {
            return 0.0;
        }
        let rata = self.moving_average();
        let variansi: f32 = self.riwayat
            .iter()
            .map(|x| (x - rata).powi(2))
            .sum::<f32>()
            / self.riwayat.len() as f32;
        variansi.sqrt()
    }

    /// Interpolasi linear: estimasi kelembaban pada waktu t
    /// antara dua titik (t0, h0) dan (t1, h1)
    fn interpolasi_linear(t0: f32, h0: f32, t1: f32, h1: f32, t: f32) -> f32 {
        if (t1 - t0).abs() < f32::EPSILON {
            return h0;
        }
        h0 + (h1 - h0) * (t - t0) / (t1 - t0)
    }

    fn tampilkan_statistik(&self) {
        println!("  ┌─────────────────────────────────────┐");
        println!("  │        STATISTIK MONITORING          │");
        println!("  ├─────────────────────────────────────┤");
        println!("  │ Moving Average  : {:.2}% RH          │", self.moving_average());
        println!("  │ Simpangan       : {:.2}%              │", self.simpangan());
        println!("  │ Total pembacaan : {}                  │", self.total_baca);
        println!("  │ Total alarm     : {}                  │", self.total_alarm);
        println!("  └─────────────────────────────────────┘");
    }
}

// ============================================================
// STRUCT: Controller
// Logika kontrol otomatis humidifier/dehumidifier
// ============================================================
#[derive(PartialEq)]
enum StatusKontrol {
    Normal,
    HumidifierOn,
    DehumidifierOn,
}

struct Controller {
    batas_min      : f32,
    batas_max      : f32,
    status         : StatusKontrol,
    humidifier_on  : bool,
    dehumidifier_on: bool,
}

impl Controller {
    fn new(min: f32, maks: f32) -> Controller {
        Controller {
            batas_min      : min,
            batas_max      : maks,
            status         : StatusKontrol::Normal,
            humidifier_on  : false,
            dehumidifier_on: false,
        }
    }

    /// Proses kontrol berdasarkan nilai kelembaban sensor aktual
    fn proses(&mut self, kelembaban: f32, monitoring: &mut MonitoringSystem) {
        if kelembaban > self.batas_max {
            self.status          = StatusKontrol::DehumidifierOn;
            self.humidifier_on   = false;
            self.dehumidifier_on = true;
            monitoring.total_alarm += 1;
            println!("  [STATUS]      Kelembaban terlalu tinggi: {:.2}% RH", kelembaban);
            println!("  [ALARM]       : ON");
            println!("  [HUMIDIFIER]  : OFF");
            println!("  [DEHUMIDIFIER]: ON");

        } else if kelembaban < self.batas_min {
            self.status          = StatusKontrol::HumidifierOn;
            self.humidifier_on   = true;
            self.dehumidifier_on = false;
            monitoring.total_alarm += 1;
            println!("  [STATUS]      Kelembaban terlalu rendah: {:.2}% RH", kelembaban);
            println!("  [ALARM]       : ON");
            println!("  [HUMIDIFIER]  : ON");
            println!("  [DEHUMIDIFIER]: OFF");

        } else {
            self.status          = StatusKontrol::Normal;
            self.humidifier_on   = false;
            self.dehumidifier_on = false;
            println!("  [STATUS]      Kelembaban normal: {:.2}% RH", kelembaban);
            println!("  [ALARM]       : OFF");
            println!("  [HUMIDIFIER]  : OFF");
            println!("  [DEHUMIDIFIER]: OFF");
        }
    }

    fn display_status(&self) {
        let status_str = match self.status {
            StatusKontrol::Normal          => "NORMAL",
            StatusKontrol::HumidifierOn    => "HUMIDIFIER AKTIF",
            StatusKontrol::DehumidifierOn  => "DEHUMIDIFIER AKTIF",
        };
        println!(
            "  [CONTROLLER] Status: {} | Humidifier: {} | Dehumidifier: {}",
            status_str,
            if self.humidifier_on   { "ON" } else { "OFF" },
            if self.dehumidifier_on { "ON" } else { "OFF" },
        );
    }
}

// ============================================================
// FUNGSI BANTU
// ============================================================

/// Cetak header aplikasi
fn cetak_header() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║     SISTEM MONITORING KELEMBABAN INDUSTRI        ║");
    println!("║     Teknik Instrumentasi - ITS                   ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();
}

/// Cetak separator
fn separator(label: &str) {
    println!("\n──────────────── {} ────────────────", label);
}

/// Baca input f32 dari terminal dengan validasi
fn input_kelembaban(prompt: &str) -> Option<f32> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let trimmed = buf.trim();

    if trimmed.eq_ignore_ascii_case("q") {
        return None;
    }

    match trimmed.parse::<f32>() {
        Ok(v)  => Some(v),
        Err(_) => {
            println!("  [ERROR] Input tidak valid, masukkan angka.");
            Some(-1.0) // trigger error sensor
        }
    }
}

// ============================================================
// MAIN
// ============================================================
fn main() {
    cetak_header();

    // Inisialisasi objek
    let mut sensor     = Sensor::new("DHT22-01");
    let mut monitoring = MonitoringSystem::new("Ruang Produksi A");
    let mut controller = Controller::new(KELEMBABAN_MIN, KELEMBABAN_MAX);

    // Set kalibrasi sensor (offset +1.5, gain 0.98)
    sensor.set_kalibrasi(1.5, 0.98);

    println!("  Sistem diinisialisasi.");
    println!("  Batas kelembaban: {:.0}% - {:.0}% RH", KELEMBABAN_MIN, KELEMBABAN_MAX);
    println!("  Kalibrasi sensor: offset={}, gain={}", sensor.offset, sensor.gain);
    println!("\n  Ketik nilai kelembaban (0-100) lalu Enter.");
    println!("  Ketik 'q' untuk keluar.\n");

    // Contoh demo interpolasi linear
    let est = MonitoringSystem::interpolasi_linear(0.0, 45.0, 10.0, 75.0, 5.0);
    println!("  [INFO] Contoh interpolasi linear pada t=5s: {:.2}% RH", est);

    let mut iterasi: u32 = 0;

    // Loop monitoring utama
    loop {
        iterasi += 1;
        separator(&format!("PEMBACAAN KE-{}", iterasi));

        // Input data sensor
        let raw = match input_kelembaban("  Masukkan nilai sensor (% RH) > ") {
            Some(v) => v,
            None    => break,  // user ketik 'q'
        };

        // Baca dan validasi sensor
        sensor.baca(raw);
        sensor.display();

        if !sensor.is_valid {
            println!("  [ERROR] Data sensor tidak valid! Lewati pembacaan ini.");
            continue;
        }

        // Hitung error terhadap nilai referensi (misal referensi = 60% RH)
        let referensi = 60.0_f32;
        let error = sensor.hitung_error(referensi);
        println!("  [KALIBRASI] Error terhadap referensi ({:.0}% RH): {:.2}%", referensi, error);

        // Simpan ke monitoring & hitung statistik
        monitoring.tambah_data(sensor.value);
        let ma = monitoring.moving_average();
        println!("  [NUMERIK] Moving Average ({} data): {:.2}% RH", WINDOW_MA, ma);
        println!("  [NUMERIK] Simpangan: {:.2}%", monitoring.simpangan());

        // Proses kontrol menggunakan nilai sensor AKTUAL (bukan moving average)
        controller.proses(sensor.value, &mut monitoring);
        controller.display_status();
    }

    // Tampilkan ringkasan akhir
    separator("RINGKASAN AKHIR");
    println!("  Sistem: {}", monitoring.nama_sistem);
    monitoring.tampilkan_statistik();
    println!("\n  Program selesai. Terima kasih!");
}
