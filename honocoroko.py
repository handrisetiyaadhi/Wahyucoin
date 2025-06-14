def jalankan(baris):
    variabel = {}

    for baris_kode in baris:
        baris_kode = baris_kode.strip()

        if baris_kode.startswith("isi"):
            # Contoh: isi nama = "Wahyu"
            _, nama, _, nilai = baris_kode.split(maxsplit=3)
            variabel[nama] = eval(nilai)

        elif baris_kode.startswith("cetak"):
            # Contoh: cetak nama
            ekspresi = baris_kode.replace("cetak", "", 1).strip()
            try:
                hasil = eval(ekspresi, {}, variabel)
            except:
                hasil = ekspresi
            print(hasil)

        else:
            print(f"Perintah tidak dikenali: {baris_kode}")

# Contoh skrip WahyuHonocoroko
kode = [
    'isi nama = "Wahyu"',
    'isi umur = 38',
    'cetak nama',
    'cetak umur',
    'cetak "Halo Dunia"'
]

jalankan(kode)
