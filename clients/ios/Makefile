# AgentsMesh iOS — build helpers.
#
# Why symlinks: `Package.swift` cannot reach files outside its root, and we
# don't want to duplicate the ~18k-line Rust-generated Swift bindings or the
# multi-hundred-MB XCFramework into the Swift package. Instead, we symlink
# them in after `build-ios-xcframework.sh` has produced them.
#
# CI uses the same recipes via `make ios-setup`.

WORKTREE := $(shell cd ../.. && pwd)
CORE_OUT := $(WORKTREE)/clients/core/ios-framework
PKG_CORE := Packages/AgentsMeshCore/Sources

.PHONY: help ios-setup xcframework link-core project check-sim clean

help:
	@echo "make xcframework   - build AgentsMeshCore.xcframework via cargo"
	@echo "make link-core     - symlink xcframework + Swift bindings into SPM tree"
	@echo "make project       - generate AgentsMesh.xcodeproj via xcodegen"
	@echo "make ios-setup     - xcframework + link-core + project (one-shot)"
	@echo "make check-sim     - build for iOS Simulator via xcodebuild"
	@echo "make clean         - remove symlinks and build artifacts"

xcframework:
	bash $(WORKTREE)/clients/core/scripts/build-ios-xcframework.sh

link-core:
	@mkdir -p $(PKG_CORE)/AgentsMeshCore/Generated
	@rm -f $(PKG_CORE)/AgentsMeshCore/Generated/AgentsMeshCore.swift
	@rm -rf $(PKG_CORE)/AgentsMeshCoreFFI/AgentsMeshCore.xcframework
	ln -s $(CORE_OUT)/Generated/AgentsMeshCore.swift \
	      $(PKG_CORE)/AgentsMeshCore/Generated/AgentsMeshCore.swift
	ln -s $(CORE_OUT)/AgentsMeshCore.xcframework \
	      $(PKG_CORE)/AgentsMeshCoreFFI/AgentsMeshCore.xcframework
	@echo "Linked Swift bindings + XCFramework into SPM tree."

project:
	xcodegen generate

ios-setup: xcframework link-core project

check-sim:
	xcodebuild \
	  -project AgentsMesh.xcodeproj \
	  -scheme AgentsMesh \
	  -destination 'generic/platform=iOS Simulator' \
	  -configuration Debug \
	  build

clean:
	rm -f $(PKG_CORE)/AgentsMeshCore/Generated/AgentsMeshCore.swift
	rm -rf $(PKG_CORE)/AgentsMeshCoreFFI/AgentsMeshCore.xcframework
	rm -rf Packages/AgentsMeshCore/.build Packages/AgentsMeshFeatures/.build
	rm -rf AgentsMesh.xcodeproj DerivedData

