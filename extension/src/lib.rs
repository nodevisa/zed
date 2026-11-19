use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree, settings::LspSettings};

struct NodevisaExtension;

impl zed::Extension for NodevisaExtension {
    fn new() -> Self {
        Self
    }

    /// Finds how to run `nodevisa build lsp`, trying in order:
    /// 1. the `lsp.nodevisa.binary` setting (`path`, and optionally `arguments` and `env`);
    /// 2. the worktree's pixi environment, when its manifest depends on `nodevisa-build`,
    ///    so that the server matches the builder version the project pins,
    ///    with pixi from `lsp.nodevisa.settings.pixi.path` or PATH,
    ///    in the environment `lsp.nodevisa.settings.pixi.environment`;
    /// 3. `nodevisa-build` on PATH (installed the `nodevisa-build` package);
    /// 4. `nodevisa build` on PATH (installed the `nodevisa` metapackage).
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let mut env = worktree.shell_env();

        let settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree).unwrap_or_default();
        if let Some(binary) = settings.binary
            && let Some(path) = binary.path
        {
            env.extend(binary.env.unwrap_or_default());
            return Ok(zed::Command {
                command: path,
                args: binary.arguments.unwrap_or_default(),
                env,
            });
        }

        let pixi_settings = PixiSettings::from(settings.settings);
        let manifest = pixi_manifest(worktree);
        let pixi = pixi_settings
            .path
            .map(|path| expand_home(path, &env))
            .or_else(|| worktree.which("pixi"));
        let (command, args): (String, Vec<&str>) = if let (Some(manifest), Some(pixi)) =
            (&manifest, pixi)
        {
            let mut args = vec!["run", "--frozen", "--manifest-path", manifest];
            if let Some(environment) = &pixi_settings.environment {
                args.extend(["--environment", environment]);
            }
            args.extend(["nodevisa-build", "lsp"]);
            (pixi, args)
        } else if let Some(build) = worktree.which("nodevisa-build") {
            (build, vec!["lsp"])
        } else if let Some(cli) = worktree.which("nodevisa") {
            (cli, vec!["build", "lsp"])
        } else {
            let pixi_hint = if manifest.is_some() {
                " (pixi is not on PATH: set `lsp.nodevisa.settings.pixi.path`)"
            } else {
                ""
            };
            return Err(format!(
                "could not find the language server. Add `nodevisa-build` to the worktree's pixi manifest{pixi_hint}, \
                 put `nodevisa-build` or `nodevisa` on PATH, or set `lsp.nodevisa.binary.path`."
            ));
        };
        Ok(zed::Command {
            command,
            args: args.iter().map(|arg| arg.to_string()).collect(),
            env,
        })
    }
}

/// The `pixi` object in `lsp.nodevisa.settings`, where an empty string is the same as none.
#[derive(Default)]
struct PixiSettings {
    path: Option<String>,
    environment: Option<String>,
}

impl From<Option<zed::serde_json::Value>> for PixiSettings {
    fn from(settings: Option<zed::serde_json::Value>) -> Self {
        let Some(pixi) = settings.as_ref().and_then(|settings| settings.get("pixi")) else {
            return Self::default();
        };
        let get = |key: &str| {
            pixi.get(key)
                .and_then(|value| value.as_str())
                .filter(|value| !value.is_empty())
                .map(str::to_string)
        };
        Self {
            path: get("path"),
            environment: get("environment"),
        }
    }
}

/// `path` with a leading `~` as the home directory, from the worktree's shell environment.
fn expand_home(path: String, env: &[(String, String)]) -> String {
    let home = env
        .iter()
        .find(|(name, _)| name == "HOME" || name == "USERPROFILE")
        .map(|(_, value)| value);
    match (path.strip_prefix('~'), home) {
        (Some(rest), Some(home)) if rest.is_empty() || rest.starts_with(['/', '\\']) => {
            format!("{home}{rest}")
        }
        _ => path,
    }
}

/// The pixi manifest at the worktree root that depends on `nodevisa-build`, if any.
fn pixi_manifest(worktree: &Worktree) -> Option<String> {
    ["pixi.toml", "pyproject.toml"]
        .into_iter()
        .find_map(|name| {
            let text = worktree.read_text_file(name).ok()?;
            text.contains("nodevisa-build")
                .then(|| format!("{}/{name}", worktree.root_path()))
        })
}

zed::register_extension!(NodevisaExtension);
