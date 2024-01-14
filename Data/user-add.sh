#!/usr/bin/bash

# Quit on error.
set -e
# Treat undefined variables as errors.
set -u

function main {
	local app_uid="${1:-}"
	local app_gid="${2:-}"

	# Change the uid
	if [[ -n "${app_uid:-}" ]]; then
		usermod -u "${app_uid}" uwsgi
	fi
	# Change the gid
	if [[ -n "${app_gid:-}" ]]; then
		groupmod -g "${app_gid}" uwsgi
	fi

	# Setup permissions on the run directory where the sockets will be
	# created, so we are sure the app will have the rights to create them.

	# Make sure the folder exists.
	mkdir /tmp/
	# Set owner.
	chown root:app /tmp
	# Set permissions.
	chmod u=rwX,g=rwX,o=--- /tmp
}

main "$@"
