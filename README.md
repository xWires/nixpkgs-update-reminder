# nixpkgs-update-reminder

A tool for NixOS to remind you with a desktop notification if you haven't updated nixpkgs in a while.

> [!IMPORTANT]
> This *only* works on NixOS, it will not work on other distros with nix installed.

## Usage

To install this on NixOS, you must be using a flake-based configuration.

1. Add this repo as an input to your flake
```nix
{
  inputs = {
    # ...
    nixpkgs-update-reminder = {
      url = "github:xWires/nixpkgs-update-reminder";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```
2. Enable the service
```nix
services.nixpkgs-update-reminder.enable = true;
```
