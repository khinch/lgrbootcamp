with (import <nixpkgs> {});
mkShell{
  buildInputs = [
    postgresql
    sqlx-cli
  ];
}
