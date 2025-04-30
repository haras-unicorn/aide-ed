let
  username = "haras";

  secrets =
    let
      hostname = "aide-ed";
    in
    {
      filePrefix = "scripts/flake/${hostname}.yaml";
      ageKeyFile = "/root/.sops.age";
      hostname = hostname;
      keys = {
        caCertFile = "ca-pub";
        postgresSslKeyFile = "postgres-ssl-priv";
        postgresSslCertFile = "postgres-ssl-pub";
        postgresInitialScript = "postgres-sql";
        keycloakPassword = "kaycloak-pass";
        keycloakPostgresPassword = "keycloak-postgres-pass";
        keycloakSslKeyFile = "keycloak-ssl-priv";
        keycloakSslCertFile = "keycloak-ssl-pub";
        userHashedPasswordFile = "user-pass-pub";
        userAuthorizedKeys = "user-ssh-pub";
        aideEdEnv = "aide-ed-env";
      };
    };

  files = {
    # shared
    caPriv = "ca-priv";
    caPub = "ca-pub";
    postgresCaSerial = "postgres-ca-srl";
    keycloakCaSerial = "keycloak-ca-srl";
    openaiApiKey = "openai-api-key";

    # instance
    postgresSslPrivate = "postgres-ssl-priv";
    postgresSslPublic = "postgres-ssl-pub";
    postgresAideEdPassword = "postgres-aide-ed-pass";
    postgresKeycloakPassword = "postgres-keycloak-pass";
    postgresUserPassword = "postgres-user-pass";
    postgresPassword = "postgres-pass";
    postgresSql = "postgres-sql";
    connectionString = "aide-ed-connection-string";
    keycloakSslPrivate = "keycloak-ssl-priv";
    keycloakSslPublic = "keycloak-ssl-pub";
    keycloakPassword = "keycloak-pass";
    userPasswordPrivate = "user-pass-priv";
    userPasswordPublic = "user-pass-pub";
    userSshPrivate = "user-ssh-priv";
    userSshPublic = "user-ssh-pub";
    aideEdEnv = "aide-ed-env";
    agePublic = "age-pub";
    agePrivate = "age-priv";
    secretsPublic = "secrets-pub";
    secretsPrivate = "secrets-priv";
  };

  rumor.imports = [
    {
      importer = "vault";
      arguments.path = "kv/aide-ed/aide-ed/${secrets.hostname}";
      arguments.allow_fail = true;
    }
    {
      importer = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.caPub;
      arguments.allow_fail = true;
    }
    {
      importer = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.postgresCaSerial;
      arguments.allow_fail = true;
    }
    {
      importer = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.keycloakCaSerial;
      arguments.allow_fail = true;
    }
    {
      importer = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.openaiApiKey;
    }
  ];

  rumor.exports = [
    {
      exporter = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.caPub;
    }
    {
      exporter = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.caPriv;
    }
    {
      exporter = "vault-file";
      arguments.path = "kv/aide-ed/shared";
      arguments.file = files.postgresCaSerial;
    }
    {
      exporter = "vault";
      arguments.path = "kv/aide-ed/aide-ed/${secrets.hostname}";
    }
    {
      exporter = "copy";
      arguments.from = files.secretsPublic;
      arguments.to = "../${secrets.filePrefix}";
    }
  ];

  rumor.generations = [
    {
      generator = "openssl-ca";
      arguments = {
        name = "aide-ed-ca";
        private = files.caPriv;
        public = files.caPub;
      };
    }
    {
      generator = "openssl";
      arguments = {
        ca_private = files.caPriv;
        ca_public = files.caPub;
        serial = files.postgresCaSerial;
        name = secrets.hostname;
        private = files.postgresSslPrivate;
        public = files.postgresSslPublic;
      };
    }
    {
      generator = "key";
      arguments = {
        name = files.postgresAideEdPassword;
        length = 32;
      };
    }
    {
      generator = "key";
      arguments = {
        name = files.postgresKeycloakPassword;
        length = 32;
      };
    }
    {
      generator = "key";
      arguments = {
        name = files.postgresUserPassword;
        length = 32;
      };
    }
    {
      generator = "key";
      arguments = {
        name = files.postgresPassword;
        length = 32;
      };
    }
    {
      generator = "moustache";
      arguments = {
        name = files.postgresSql;
        variables = {
          POSTGRES_PASS = files.postgresPassword;
          AIDE_ED_POSTGRES_PASS = files.postgresAideEdPassword;
          KEYCLOAK_POSTGRES_PASS = files.postgresKeycloakPassword;
          USER_POSTGRES_PASS = files.postgresUserPassword;
        };
        template = ''
          ALTER USER postgres WITH PASSWORD '{{POSTGRES_PASS}}';

          CREATE USER ${username} PASSWORD '{{USER_POSTGRES_PASS}}';
          CREATE USER aide-ed PASSWORD '{{AIDE_ED_POSTGRES_PASS}}';
          CREATE USER keycloak PASSWORD '{{KEYCLOAK_POSTGRES_PASS}}';

          CREATE DATABASE aide-ed;
          ALTER DATABASE aide-ed OWNER TO aide-ed;
          \c aide-ed
          GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${username};
          GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ${username};
          GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO ${username};

          CREATE DATABASE keycloak;
          ALTER DATABASE keycloak OWNER TO keycloak;
          \c keycloak
          GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${username};
          GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ${username};
          GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO ${username};
        '';
        renew = true;
      };
    }
    {
      generator = "key";
      arguments = {
        name = files.keycloakPassword;
        length = 32;
      };
    }
    {
      generator = "openssl";
      arguments = {
        ca_private = files.caPriv;
        ca_public = files.caPub;
        serial = files.keycloakCaSerial;
        name = secrets.hostname;
        private = files.keycloakSslPrivate;
        public = files.keycloakSslPublic;
      };
    }
    {
      generator = "mkpasswd";
      arguments = {
        public = files.userPasswordPublic;
        private = files.userPasswordPrivate;
      };
    }
    {
      generator = "ssh-keygen";
      arguments = {
        name = secrets.hostname;
        public = files.userSshPublic;
        private = files.userSshPrivate;
      };
    }
    {
      generator = "moustache";
      arguments = {
        name = files.connectionString;
        variables = {
          AIDE_ED_POSTGRES_PASS = files.postgresAideEdPassword;
        };
        template = "postgres://"
          + "aide-ed:{{AIDE_ED_POSTGRES_PASS}}"
          + "@localhost:5432/aide-ed?sslmode=disable";
        renew = true;
      };
    }
    {
      generator = "env";
      arguments = {
        name = files.aideEdEnv;
        variables = {
          AIDE_ED_DB_DOMAIN = "localhost";
          AIDE_ED_DB_PORT = "5432";
          AIDE_ED_DB_USER = "aide-ed";
          AIDE_ED_DB_PASSWORD = files.postgresAideEdPassword;
          AIDE_ED_DB_NAME = "aide-ed";

          AIDE_ED_OPENAI_URL = "https://api.deepseek.com/v1";
          AIDE_ED_OPENAI_API_KEY = files.openaiApiKey;

          AIDE_ED_KEYCLOAK_URL = "localhost:8080";
          AIDE_ED_KEYCLOAK_USER = "aide-ed";
          AIDE_ED_KEYCLOAK_PASSWORD = files.keycloakPassword;
        };
        renew = true;
      };
    }
    {
      generator = "age";
      arguments = {
        private = files.agePrivate;
        public = files.agePublic;
      };
    }
    {
      generator = "sops";
      arguments = {
        age = files.agePublic;
        private = files.secretsPrivate;
        public = files.secretsPublic;
        secrets = {
          ${secrets.keys.caCertFile} = files.caPub;
          ${secrets.keys.postgresSslKeyFile} = files.postgresSslPrivate;
          ${secrets.keys.postgresSslCertFile} = files.postgresSslPublic;
          ${secrets.keys.postgresInitialScript} = files.postgresSql;
          ${secrets.keys.keycloakPassword} = files.keycloakPassword;
          ${secrets.keys.keycloakPostgresPassword} = files.postgresKeycloakPassword;
          ${secrets.keys.keycloakSslCertFile} = files.keycloakSslPublic;
          ${secrets.keys.keycloakSslKeyFile} = files.keycloakSslPrivate;
          ${secrets.keys.userHashedPasswordFile} = files.userPasswordPublic;
          ${secrets.keys.userAuthorizedKeys} = files.userSshPublic;
          ${secrets.keys.aideEdEnv} = files.aideEdEnv;
        };
        renew = true;
      };
    }
  ];
in
{
  flake.lib.secrets."raspberryPi4-aarch64-linux" = secrets;

  flake.lib.rumor."raspberryPi4-aarch64-linux" = rumor;
}
