const completion: Fig.Spec = {
  name: "ferrishot",
  description: "A powerful screenshot app",
  options: [
    {
      name: ["-r", "--region"],
      description: "Open with a region pre-selected",
      isRepeatable: true,
      args: {
        name: "region",
        isOptional: true,
      },
    },
    {
      name: ["-a", "--accept-on-select"],
      description: "Accept on first selection",
      isRepeatable: true,
      args: {
        name: "accept_on_select",
        isOptional: true,
        suggestions: [
          {
            name: "copy",
            description: "Copy image to the clipboard",
          },
          {
            name: "save",
            description: "Save image to a file",
          },
          {
            name: "upload",
            description: "Upload image to the internet",
          },
        ],
      },
    },
    {
      name: ["-d", "--delay"],
      description: "Wait this long before launch",
      isRepeatable: true,
      args: {
        name: "delay",
        isOptional: true,
      },
    },
    {
      name: ["-s", "--save-path"],
      description: "Save image to path",
      isRepeatable: true,
      args: {
        name: "save_path",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-C", "--config-file"],
      description: "Use the provided config file",
      isRepeatable: true,
      args: {
        name: "config_file",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: "--log-level",
      description: "Choose a miniumum level at which to log",
      hidden: true,
      isRepeatable: true,
      args: {
        name: "log_level",
        isOptional: true,
      },
    },
    {
      name: "--log-file",
      description: "Path to the log file",
      hidden: true,
      isRepeatable: true,
      args: {
        name: "log_file",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: "--log-filter",
      description: "Filter for specific Rust module or crate, instead of showing logs from all crates",
      hidden: true,
      isRepeatable: true,
      args: {
        name: "log_filter",
        isOptional: true,
      },
    },
    {
      name: ["-l", "--last-region"],
      description: "Use last region",
      exclusiveOn: [
        "-r",
        "--region",
      ],
    },
    {
      name: ["-D", "--dump-default-config"],
      description: "Write the default config to /home/e/.config/ferrishot.kdl",
    },
    {
      name: ["-S", "--silent"],
      description: "Run in silent mode",
    },
    {
      name: ["-j", "--json"],
      description: "Print in JSON format",
      exclusiveOn: [
        "-S",
        "--silent",
      ],
    },
    {
      name: "--log-stderr",
      description: "Log to standard error instead of file",
      exclusiveOn: [
        "-S",
        "--silent",
      ],
    },
    {
      name: "--debug",
      description: "Launch in debug mode (F12)",
    },
    {
      name: ["-h", "--help"],
      description: "Print help (see more with '--help')",
    },
    {
      name: ["-V", "--version"],
      description: "Print version",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
