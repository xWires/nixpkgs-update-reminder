self: { config, lib, pkgs, ... }:
{
  options.services.nixpkgs-update-reminder.enable = lib.mkEnableOption "Send a notification if nixpkgs hasn't been updated recently";

  config = lib.mkIf config.services.nixpkgs-update-reminder.enable {
    systemd.user.packages = [ self.packages.${pkgs.system}.default ];

    systemd.user.timers.nixpkgs-update-reminder.wantedBy = [ "timers.target" ];
  };
}
