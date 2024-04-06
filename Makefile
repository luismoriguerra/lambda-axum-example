


start:
	cargo lambda watch

build:
	cargo lambda build

deploy:
	cargo lambda build --release
	cargo lambda deploy --enable-function-url

remove:
	aws lambda delete-function --function-name cargoaxum

log:
	sam logs --cw-log-group /aws/lambda/cargoaxum
	
logt:
	sam logs --cw-log-group /aws/lambda/cargoaxum --tail