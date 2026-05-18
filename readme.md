TUTORIAL 1
1. ![Eksperimen 1.2](images/experiment_1.2.png)
Teks hey hey akan tercetak sebelum howdy! (atau setidanya sebelum program selesai), hal ini terjadi karena spawner.spawn(...) cuma berfungsi mendaftarkan/memasukkan tugas (task) asinkron ke dalam antrean (queue), tapi tugas itu sendiri belum dijalankan. Baris hey hey adalah kode sinkron (biasa) yang langsung dijalankan saat itu juga oleh fungsi utama. Tugas asinkron (howdy! dan done!) baru benar-benar mulai dieksekusi ketika baris paling bawah, yaitu executor.run();, dipanggil

2. ![Eksperimen 1.3](images/experiment_1.3.png)
Executor (melalui fungsi executor.run()) dirancang untuk terus mendengarkan dan mengeksekusi tugas dari sebuah antrean saluran (channel queue). Dia akan terus berjalan selama sender (yaitu Spawner) masih eksis dan terhubung. Fungsi drop(spawner) berguna untuk menutup/mematikan sender tersebut, yang memberikan sinyal kepada Executor bahwa "Tidak ada tugas baru lagi yang akan datang". Tanpa drop(spawner), Executor akan mengira akan ada tugas baru lagi di masa depan, sehingga ia menunggu selamanya


TUTORIAL 2
1. ![Eksperimen 2.1 - server](images/2.1_server.png.png)
![Eksperimen 2.1 - client 1](images/2.1_client_1.png)
![Eksperimen 2.1 - client 2](images/2.1_client_2.png)
![Eksperimen 2.1 - client 3](images/2.1_client_3.png)
Untuk menjalankan aplikasi ini, jalankan perintah cargo run --bin server di satu terminal, lalu buka tiga terminal baru dan jalankan cargo run --bin client di masing-masing terminal tersebut. Saat kita mengetik dan mengirim pesan dari salah satu klien, pesan itu akan dikirim ke server melalui jaringan WebSocket. Server kemudian bertindak sebagai pusat siaran (broadcaster) yang langsung meneruskan pesan tersebut ke semua klien lain yang sedang terhubung, sehingga obrolan muncul di layar semua orang secara real-time.

