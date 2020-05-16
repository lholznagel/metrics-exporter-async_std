VERSION := 0.1.0

version:
	sed -i '0,/version = .*/s/version = .*/version = "${VERSION}"/g' Cargo.toml
	git tag -s ${VERSION} -m "${VERSION}"
	git push origin master --tags