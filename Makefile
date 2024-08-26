# Project Setup
PROJECT_NAME := configuration-demo-backend
PROJECT_REPO := github.com/jonasz-lasut/$(PROJECT_NAME)

# NOTE(hasheddan): the platform is insignificant here as Configuration package
# images are not architecture-specific. We constrain to one platform to avoid
# needlessly pushing a multi-arch image.
PLATFORMS ?= linux_amd64
-include build/makelib/common.mk

# ====================================================================================
# Setup Kubernetes tools

KUBECTL_VERSION=v1.30.2
KIND_VERSION=v0.23.0
UP_VERSION = v0.32.0
UP_CHANNEL = stable
UPTEST_VERSION = v0.11.1

-include build/makelib/k8s_tools.mk
# ====================================================================================
# Setup XPKG
XPKG_DIR = $(shell pwd)
XPKG_IGNORE = .github/workflows/*.yaml,.github/workflows/*.yml,examples/*.yaml,.work/uptest-datasource.yaml,test/provider/*.yaml,gitops/*.yaml
XPKG_REG_ORGS ?= xpkg.upbound.io/judasz
XPKG_REG_ORGS_NO_PROMOTE ?= xpkg.upbound.io/judasz
XPKGS = $(PROJECT_NAME)
-include build/makelib/xpkg.mk

CROSSPLANE_NAMESPACE = upbound-system
CROSSPLANE_ARGS = "--enable-usages"
-include build/makelib/local.xpkg.mk
-include build/makelib/controlplane.mk

# ====================================================================================
# Targets

# run `make help` to see the targets and options

# We want submodules to be set up the first time `make` is run.
# We manage the build/ folder and its Makefiles as a submodule.
# The first time `make` is run, the includes of build/*.mk files will
# all fail, and this target will be run. The next time, the default as defined
# by the includes will be run instead.
fallthrough: submodules
	@echo Initial setup complete. Running make again . . .
	@make

# Update the submodules, such as the common build scripts.
submodules:
	@git submodule sync
	@git submodule update --init --recursive

# We must ensure up is installed in tool cache prior to build as including the k8s_tools machinery prior to the xpkg
# machinery sets UP to point to tool cache.
build.init: $(UP)

# ====================================================================================
# End to End Testing

# This target requires the following environment variables to be set:
uptest: $(UPTEST) $(KUBECTL) $(KUTTL)
	@$(INFO) running automated tests
	@KUBECTL=$(KUBECTL) KUTTL=$(KUTTL) CROSSPLANE_NAMESPACE=$(CROSSPLANE_NAMESPACE) $(UPTEST) e2e examples/atlas-xr.yaml --setup-script=test/setup.sh --default-timeout=2400 || $(FAIL)
	@$(OK) running automated tests

e2e: build controlplane.up local.xpkg.deploy.configuration.$(PROJECT_NAME) uptest

render:
	crossplane beta render examples/atlas-xr.yaml apis/composition.yaml examples/functions.yaml -r

yamllint:
	@$(INFO) running yamllint
	@yamllint ./apis || $(FAIL)
	@$(OK) running yamllint

# This target requires passing XPKG_TOKEN
login:
	$(UP) login --token $(XPKG_TOKEN)

.PHONY: uptest e2e render yamllint
