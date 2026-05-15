[README.md](https://github.com/user-attachments/files/27790128/README.md)
# 💧 Humidity Monitoring and Alarm System

Sistem monitoring kelembaban udara berbasis **Rust** dengan tampilan dashboard interaktif di browser.

---

## 👥 Anggota Kelompok

| Nama | NRP |
|------|-----|
| Galang Romadhan | 2042251008 |
| Ruben Tambarta Sitio | 2042251029 |

**Mata Kuliah:** Algoritma Pemrograman  
**Program Studi:** D-4 Teknologi Rekayasa Instrumentasi Institut Teknologi Sepuluh Nopember (ITS)

---

## 📋 Deskripsi Program

Program ini merupakan sistem monitoring kelembaban udara yang dibangun menggunakan bahasa pemrograman Rust. Data kelembaban dibaca dari sensor humidity dan ditampilkan secara visual melalui dashboard berbasis HTML di browser.

### Fitur Utama:
- 📊 Dashboard monitoring kelembaban real-time
- 🎮 Sistem control Humidifier dan Dehumidifier ketika kelembaban kurang atau melebihi batas normal
- ⚠️ Sistem safety alarm ketika kelembaban melebihi batas normal
- 🖥️ Tampilan antarmuka berbasis web (HTML)

---

## 🛠️ Teknologi yang Digunakan

- **Bahasa Pemrograman:** Rust
- **Package Manager:** Cargo
- **Tampilan:** HTML (dashboard_kelembaban.html)
- **Sensor:** Humidity Sensor

---

## 🚀 Cara Menjalankan Program

### Prasyarat
Pastikan sudah menginstall:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)

### Langkah-langkah

1. **Clone repository ini**
   ```bash
   git clone https://github.com/ben-911-bhap/Humidity-Monitoring-and-Ala.git
   cd Humidity-Monitoring-and-Ala
   ```

2. **Build program**
   ```bash
   cargo build
   ```

3. **Jalankan program**
   ```bash
   cargo run
   ```

4. **Buka dashboard** di browser:
   ```
   Buka file src/dashboard_kelembaban.html
   ```

---

## 📁 Struktur Project

```
Humidity-Monitoring-and-Ala/
├── Cargo.toml                      # Konfigurasi project Rust
├── Cargo.lock                      # Lock file dependencies
├── .gitignore                      # File yang diabaikan Git
└── src/
    ├── main.rs                     # Program utama Rust
    └── dashboard_kelembaban.html   # Tampilan dashboard
```

---

## 📌 Catatan

Program ini dibuat sebagai tugas mata kuliah **Algoritma Pemrograman**.
