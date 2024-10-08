# configuration-demo-backend

Requirements:
- secret in Secret Manager for database with a key called password


To build a Configuration package, use the Crossplane CLI:
```shell
# Just builds the package. It will go through the repo and collect yaml files.
# Furthermore it will by default check for examples in `./examples` and add them to the package.
crossplane xpkg build

# You can specify various options for the build command, if needed
crossplane xpkg build --package-root=. --package-file=test-package.xpkg --examples-root=./examples
```

(Optionally) Login to the default xpkg.upbound.io registry:
```shell
crossplane xpkg login
```

Push the package to a registry:
```shell
crossplane xpkg push -f test-package.xpkg your-org/your-repo:v1.0.0
```

More info on the Crossplane CLI can be found [here][xp-cli].

Check out the Crossplane docs for more information on [creating] Configurations.

[Configuration]: https://docs.crossplane.io/latest/concepts/packages
[creating]: https://docs.crossplane.io/latest/concepts/packages/#create-a-configuration
[xp-cli]: https://docs.crossplane.io/latest/cli/command-reference/
