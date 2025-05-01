{ pkgs
, self
, ...
}:

{
  seal.defaults.devShell = "dev";
  integrate.devShell = {
    nixpkgs.config = {
      allowUnfree = true;
    };

    devShell = pkgs.mkShell {
      shellHook = ''
        dockerCompose="${./dockerCompose.nu}"

        $dockerCompose up

        AIDE_ED_HOST="127.0.0.1";
        export AIDE_ED_HOST
        echo "AIDE_ED_HOST is set to $AIDE_ED_HOST"

        while true; do
          AIDE_ED_HTTP_PORT=$(( RANDOM % (65535 - 32768 + 1) + 32768 ))
          ss -Hlnu | grep -q ":$PORT " || break
        done
        export AIDE_ED_HTTP_PORT
        echo "AIDE_ED_HTTP_PORT is set to $AIDE_ED_HTTP_PORT"

        while true; do
          AIDE_ED_HTTPS_PORT=$(( RANDOM % (65535 - 32768 + 1) + 32768 ))
          ss -Hlnu | grep -q ":$PORT " || break
        done
        export AIDE_ED_HTTPS_PORT
        echo "AIDE_ED_HTTPS_PORT is set to $AIDE_ED_HTTPS_PORT"

        AIDE_ED_DB_HOST="$($dockerCompose postgres host)"
        export AIDE_ED_DB_HOST
        echo "AIDE_ED_DB_HOST is set to $AIDE_ED_DB_HOST"

        AIDE_ED_DB_PORT="$($dockerCompose postgres port)"
        export AIDE_ED_DB_PORT
        echo "AIDE_ED_DB_PORT is set to $AIDE_ED_DB_PORT"

        AIDE_ED_DB_USER="$($dockerCompose postgres user)"
        export AIDE_ED_DB_USER
        echo "AIDE_ED_DB_USER is set to $AIDE_ED_DB_USER"

        AIDE_ED_DB_PASSWORD="$($dockerCompose postgres password)"
        export AIDE_ED_DB_PASSWORD
        echo "AIDE_ED_DB_PASSWORD is set to $AIDE_ED_DB_PASSWORD"

        AIDE_ED_DB_NAME="$($dockerCompose postgres database)"
        export AIDE_ED_DB_NAME
        echo "AIDE_ED_DB_NAME is set to $AIDE_ED_DB_NAME"

        AIDE_ED_OPENAI_URL="https://$($dockerCompose openai host):$($dockerCompose openai port)/v1"
        export AIDE_ED_OPENAI_URL
        echo "AIDE_ED_OPENAI_URL is set to $AIDE_ED_OPENAI_URL"

        AIDE_ED_OPENAI_API_KEY="$($dockerCompose openai api-key)"
        export AIDE_ED_OPENAI_API_KEY
        echo "AIDE_ED_OPENAI_API_KEY is set to $AIDE_ED_OPENAI_API_KEY"

        AIDE_ED_KEYCLOAK_URL="$($dockerCompose keycloak url)"
        export AIDE_ED_KEYCLOAK_URL
        echo "AIDE_ED_KEYCLOAK_URL is set to $AIDE_ED_KEYCLOAK_URL"

        AIDE_ED_KEYCLOAK_USER="$($dockerCompose keycloak user)"
        export AIDE_ED_KEYCLOAK_USER
        echo "AIDE_ED_KEYCLOAK_USER is set to $AIDE_ED_KEYCLOAK_USER"

        AIDE_ED_KEYCLOAK_PASSWORD="$($dockerCompose keycloak password)"
        export AIDE_ED_KEYCLOAK_PASSWORD
        echo "AIDE_ED_KEYCLOAK_PASSWORD is set to $AIDE_ED_KEYCLOAK_PASSWORD"

        DATABASE_URL="postgres://$AIDE_ED_DB_USER:$AIDE_ED_DB_PASSWORD@$AIDE_ED_DB_HOST:$AIDE_ED_DB_PORT/$AIDE_ED_DB_NAME?sslmode=disable"
        export DATABASE_URL
        echo "DATABASE_URL is set to $DATABASE_URL"

        $dockerCompose ready
      '';

      inputsFrom = [
        (self.lib.vcs.mkDevShell pkgs)
        (self.lib.scripts.mkDevShell pkgs)
        (self.lib.rust.mkDevShell pkgs)
        self.devShells.${pkgs.system}.docs
        (self.lib.format.mkDevShell pkgs)
        (self.lib.lint.mkDevShell pkgs)
        (self.lib.tools.mkDevShell pkgs)
        (self.lib.lsp.mkDevShell pkgs)
      ];
    };
  };
}
