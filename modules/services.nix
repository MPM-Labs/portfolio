{ inputs, ... }: {
  imports = [
    inputs.process-compose-flake.flakeModule
  ];
  perSystem = { self', ... }: {
    process-compose.dev = {
      imports = [
        inputs.services-flake.processComposeModules.default
      ];
      services.postgres."pg1" = {
        enable = true;
        initialScript.before = ''
          CREATE USER postgres WITH password 'postgres';
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