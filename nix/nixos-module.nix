{ config, lib, pkgs, ... }:

let
  cfg = config.dotfiles;
in
{
  options.dotfiles = {
    enable = lib.mkEnableOption "dotfiles system-level configuration";

    user = lib.mkOption {
      type = lib.types.str;
      description = "Primary user account to configure.";
      example = "josiah";
    };

    directory = lib.mkOption {
      type = lib.types.path;
      default = "/home/${cfg.user}/.dotfiles";
      description = "Path to the cloned dotfiles repository.";
    };

    sshKeySync = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Whether to enable periodic SSH key sync from GitHub.";
      };

      githubUser = lib.mkOption {
        type = lib.types.str;
        default = "josiahbull";
        description = "GitHub username to fetch SSH public keys from.";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    # System packages
    environment.systemPackages = with pkgs; [
      zsh
      git
      gnupg
      curl
      tmux
      nano
      gnutar
    ];

    # Enable zsh system-wide
    programs.zsh.enable = true;

    # Set the user's default shell to zsh
    users.users.${cfg.user}.shell = pkgs.zsh;

    # Systemd timer for SSH key sync (replaces cron)
    systemd.services."dotfiles-ssh-key-sync" = lib.mkIf cfg.sshKeySync.enable {
      description = "Sync SSH authorized_keys from GitHub";
      serviceConfig = {
        Type = "oneshot";
        User = cfg.user;
        Environment = "GITHUB_SSH_USER=${cfg.sshKeySync.githubUser}";
        ExecStart = "${cfg.directory}/scripts/sync-ssh-keys.sh";
        LogsDirectory = "";
        StandardOutput = "append:/home/${cfg.user}/.local/log/ssh-key-sync.log";
        StandardError = "append:/home/${cfg.user}/.local/log/ssh-key-sync.log";
      };
      # Ensure log directory exists
      preStart = "${pkgs.coreutils}/bin/mkdir -p /home/${cfg.user}/.local/log";
    };

    systemd.timers."dotfiles-ssh-key-sync" = lib.mkIf cfg.sshKeySync.enable {
      description = "Run SSH key sync every 6 hours";
      wantedBy = [ "timers.target" ];
      timerConfig = {
        OnBootSec = "5min";
        OnUnitActiveSec = "6h";
        Persistent = true;
      };
    };
  };
}
