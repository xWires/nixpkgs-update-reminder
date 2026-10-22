{ lib, stdenv, pkgs }:
pkgs.rustPlatform.buildRustPackage {
  pname = "nixpkgs-update-reminder";
  version = "1.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    install -Dm644 nixpkgs-update-reminder.service $out/lib/systemd/user/nixpkgs-update-reminder.service
    install -Dm644 nixpkgs-update-reminder.timer $out/lib/systemd/user/nixpkgs-update-reminder.timer

    substituteInPlace $out/lib/systemd/user/nixpkgs-update-reminder.service \
      --replace-fail "@bin@" "$out/bin/nixpkgs-update-reminder"
  '';
}
