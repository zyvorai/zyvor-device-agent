#!/usr/bin/env sh
# Copyright 2026 Zyvor AI Labs · https://zyvor.dev
# SPDX-License-Identifier: LicenseRef-Zyvor-Production-2.0

set -eu
printf '{"sensor":"demo-temperature","value":%s,"unit":"celsius","quality":"simulated"}\n' "${ZYVOR_DEMO_TEMP:-24.8}"
