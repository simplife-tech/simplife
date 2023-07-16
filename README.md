docker build . --build-arg https_proxy=http://host.docker.internal:1087 -f account-service/Dockerfile -t devynchou/simplife.account.service:test   
docker build . --build-arg https_proxy=http://host.docker.internal:1087 -f ledger-service/Dockerfile -t devynchou/simplife.ledger.service:test
docker build . -f agenda-service/Dockerfile -t devynchou/simplife.agenda.service:test    