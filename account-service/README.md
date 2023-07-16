cargo run --bin account-service -- --db mysql://root:sjkzyflzsz@101.35.135.201:3306/simplife_test --redis redis://:V36Sv8o7ttCzaYSC@101.35.135.201:6380/

cargo run --bin account-service -- --listen-ip 0.0.0.0 --db mysql://root:sjkzyflzsz@101.35.135.201:3306/simplife_test --redis redis://:V36Sv8o7ttCzaYSC@101.35.135.201:6380/

r = requests.post("http://localhost:27001/login", json={"mobile":"13913172001","password":"zyflzsz"})