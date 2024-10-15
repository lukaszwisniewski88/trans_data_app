{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.DATABASE_URL = "postgresql://developer:developer@localhost:5432/trans";
  env.ROCKET_DATABASES = "{trans_api = {url = \"postgresql://developer:developer@localhost:5432/trans\"}}";


  # https://devenv.sh/packages/
  packages = [ pkgs.git ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';
  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  languages.javascript = {
    enable = true;
    pnpm.enable = true;
    corepack.enable = true;
  };
  services.postgres = {
    enable = true;
    settings = {
      log_connections = true;
      log_statement = "all";
      log_disconnections = true;
    };
    listen_addresses = "0.0.0.0";
    extensions = extensions: [ extensions.postgis ];
    initialScript = ''
          CREATE EXTENSION IF NOT EXISTS postgis;
          CREATE USER developer with password 'developer';
        CREATE DATABASE trans OWNER developer;
      grant all privileges on database trans to developer;
    '';
  };

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
