# Some explanations

## English language
This demo is implemented by Rocket 0.5 and Diesel 2.1.5
1. Created REST APIs and implemented the function of adding, deleting, modifying, and querying
2. Basic packaging has been carried out
3. Implemented database migration
4. Implemented Basic authorization and provided examples
5. Deploy the program to the server

## Chinese language
这个demo是由Rocket 0.5 和 diesel 2.1.5实现

1. 创建了REST APIs,实现了增删改查功能
2. 进行了基本封装
3. 实现了数据库的迁移
4. 实现了Basic授权，并提供实例
5. 部署程序到服务器上


## database
sudo apt-get install libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite


diesel setup --database-url ./database.sqlite
diesel migration generate create_runstaceans
diesel migration run --database-url=database.sqlite
diesel migration list --database-url=database.sqlite

### release
> cargo build --release
> scp ./target/release/rocket-app >>>
> ROCKET_DATABASES={sqlite = { url = "./database.sqlite" }} ./rocket-app
> ./rocket-app

> cd /etc/systemd/system
> vim rocket-app.service
paste it!
> systemctl start rocket-app

### proxy
nginx 
it is very easy, so no notes.