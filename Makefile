ADVERTISER_ID=424242
PUBLISHER_EXTERNAL_ID=242424

run-local-config:
	cargo build
	./target/debug/tiger-claw -n local -v -c tests.local.toml

run-local-config-cli-override:
	cargo build
	./target/debug/tiger-claw -n local -v -e $(PUBLISHER_EXTERNAL_ID) -a $(ADVERTISER_ID) -c tests.local.toml


run-dev-config:
	cargo build
	./target/debug/tiger-claw -n dev -v -c tests.dev.toml

run-dev-config-cli-override:
	cargo build
	./target/debug/tiger-claw -n dev -v -e $(PUBLISHER_EXTERNAL_ID) -a $(ADVERTISER_ID) -c tests.dev.toml


run-staging-config:
	cargo build
	./target/debug/tiger-claw -n staging -v  -c tests.staging.toml

run-staging-config-override:
	cargo build
	./target/debug/tiger-claw -n staging -v -e $(PUBLISHER_EXTERNAL_ID) -a $(ADVERTISER_ID) -c tests.staging.toml

sas-data-import-ssm-dev:
	aws ssm start-session --profile org-adm-springfield-dev-poweruser \
	--target i-07f0ee9c602cd1f6b --document-name AWS-StartPortForwardingSessionToRemoteHost \
	--parameters '{ "portNumber":["80"], "localPortNumber":["8181"], "host":["sas-data-import.cloudmap.eu-west-1.springfield-dev.awin-aws.com"]}'

growth-migration-ssm-dev:
	aws ssm start-session --profile org-adm-springfield-dev-poweruser \
	--target i-07f0ee9c602cd1f6b --document-name AWS-StartPortForwardingSessionToRemoteHost \
	--parameters '{ "portNumber":["80"], "localPortNumber":["8080"], "host":["growth-account-migration-service.cloudmap.eu-west-1.springfield-dev.awin-aws.com"]}'
