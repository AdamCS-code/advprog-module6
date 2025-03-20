Milestone 1: Single threaded web server
Pada milestode 3 ini, Program rust di `main.rs` adalah web server sederhana yang dapat menerima request. Disini program tersebut membuka tcp connection pada port `7878` pada localhost. Kemudian, program mendengarkan incomming connection dan print request yang diberikan. Ketika kita melakukan pencarian di browser `localhost:7878` maka kita dapat melihat di terminal terdapat request yang diberikan menggunakan protokol http untuk tipe get request. Tidak hanya itu, kita dapat melihat elemen lain pada request tersebut seperti user-agent, connection, dll.

Milestone 2: Returning HTML
Pada milestone 2 ini, kita akan mengembalikan response bersamaan dengan sebuah file html sederhana. Pada standard library rust, terdapat module fs yang dapat kita manfaatkan untuk membaca file html yang akan kita kirim. Kemudian, kita dapat menambahkan isi html tersebut di response.

Milestone 3: Validating request and selectively responding
Pada milestone 3 ini, kita mencoba mengembangkan rooting pada web server. Jadi, untuk url yang tidak diketahui, kita dapat mengembalikan pesan seperti `404 NOT FOUND` pada user. Untuk itu, kita perlu membaca line pertama dari request tersebut. Kemudian, program membaca dan menentukan apakah get request tersebut memang telah ditentukan. Apabila, user mengakses url yang tidak diketahui, maka kita dapat memberikan `404 NOT FOUND`.

Milestone 4: Simulation slow response
Pada milestone 4 ini, kita mencoba mensimulasikan slow response pada user. Dapat kita lihat, karena ini single-threaded maka selama menunggu response dari web server, kita tidak dapat berkomunikasi dengan user sebelumnya. Ini tentunya bukan hal yang bijak. Bayangkan ada 1000 user yang mengakses url `localhost:7878/sleep` maka user selanjutnya harus menununggu 1000 * 10 detik atau kurang lebih 2 jam. Tentunya secara performance ini sangat buruk.

Milestone 5: Multithreaded Server
Pada milestone 5 ini, kita mencoba menjawab permasalahan pada milestone 4. Solusinya cukup simple, kita dapat mengimplementasikan multi-threading pada web server ini. Kita membuat thread pool atau kumpulan thread sebagai virtual user. Thread dalam pool tersebut akan melayani user yang mengakses url `localhost:7878`. Kita mengabstraksikan thread sebagai seorang worker yang melayani user. Implementasi thread pool didasarkan pada concern DDOS attack yang mungkin terjadi apabila jumlah thread yang dispawn tidak dibatasi. Jadi, web server tersebut dapat mengerjakan request user secara concurrent. 
