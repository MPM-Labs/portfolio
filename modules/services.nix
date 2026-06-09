{ inputs, ... }: {
  imports = [
    inputs.process-compose-flake.flakeModule
  ];
  perSystem = { self', pkgs, ... }: {
    process-compose.dev = {
      imports = [
        inputs.services-flake.processComposeModules.default
      ];
      services.postgres."pg1" = {
        enable = true;
        package = pkgs.postgresql_18;
        initialScript.before = ''
          CREATE USER postgres WITH password 'postgres';
        '';
        initialScript.after = ''
          \c db
          GRANT ALL PRIVILEGES ON SCHEMA public TO postgres;
          GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO postgres;
          ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO postgres;
        '';
        initialDatabases = [
          {
            name = "db";
          }
        ];
      };
    };
    apps.default = {
      type = "app";
      program = self'.packages.dev;
    };
  };
}