cargo run --bin ledger-service -- --listen-port 27002 --db mysql://root:sjkzyflzsz@101.35.135.201:3306/simplife_test --redis redis://:V36Sv8o7ttCzaYSC@101.35.135.201:6380/

cargo run --bin ledger-service -- --listen-ip 0.0.0.0 --listen-port 27002 --db mysql://root:sjkzyflzsz@101.35.135.201:3306/simplife_test --redis redis://:V36Sv8o7ttCzaYSC@101.35.135.201:6380/

r = requests.get("http://localhost:27002/ledger/list?access_key=jfOJthJqrTPfoINFT9t1mZnMeGtKaSRA&date_start=0&date_end=1689349419&pn=1&ps=1000&kind=family")