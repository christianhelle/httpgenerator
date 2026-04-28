# Changelog

## [Unreleased](https://github.com/christianhelle/httpgenerator/tree/HEAD)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/1.1.0-preview.18...HEAD)

**Merged pull requests:**

- chore\(deps\): update dependency microsoft.net.test.sdk to 18.5.1 [\#370](https://github.com/christianhelle/httpgenerator/pull/370) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency ruby to v4.0.3 [\#369](https://github.com/christianhelle/httpgenerator/pull/369) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency coverlet.collector to v10 [\#368](https://github.com/christianhelle/httpgenerator/pull/368) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/vscode to v1.116.0 [\#366](https://github.com/christianhelle/httpgenerator/pull/366) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/upload-pages-artifact action to v5 [\#364](https://github.com/christianhelle/httpgenerator/pull/364) ([renovate[bot]](https://github.com/apps/renovate))
- Upgrade Spectre.Console.Cli from 0.53.1 to 0.55.0 [\#363](https://github.com/christianhelle/httpgenerator/pull/363) ([christianhelle](https://github.com/christianhelle))
- chore\(deps\): update actions/github-script action to v9 [\#362](https://github.com/christianhelle/httpgenerator/pull/362) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/vscode to v1.115.0 [\#360](https://github.com/christianhelle/httpgenerator/pull/360) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency microsoft.net.test.sdk to 18.4.0 [\#359](https://github.com/christianhelle/httpgenerator/pull/359) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v24.12.2 [\#357](https://github.com/christianhelle/httpgenerator/pull/357) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency community.visualstudio.toolkit.16 to 16.0.551 [\#356](https://github.com/christianhelle/httpgenerator/pull/356) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/configure-pages action to v6 [\#355](https://github.com/christianhelle/httpgenerator/pull/355) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update codecov/codecov-action action to v6 [\#354](https://github.com/christianhelle/httpgenerator/pull/354) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/deploy-pages action to v5 [\#353](https://github.com/christianhelle/httpgenerator/pull/353) ([renovate[bot]](https://github.com/apps/renovate))

## [1.1.0-preview.18](https://github.com/christianhelle/httpgenerator/tree/1.1.0-preview.18) (2026-03-21)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/1.0.1-preview.17...1.1.0-preview.18)

**Implemented enhancements:**

- GenerateSampleJson produces empty JSON for allOf/oneOf/anyOf composed schemas [\#313](https://github.com/christianhelle/httpgenerator/issues/313)
- Add support for OpenAPI v3.1 [\#349](https://github.com/christianhelle/httpgenerator/pull/349) ([christianhelle](https://github.com/christianhelle))
- Deduplicate filenames in OneRequestPerFile mode [\#325](https://github.com/christianhelle/httpgenerator/pull/325) ([christianhelle](https://github.com/christianhelle))
- Regression tests for query parameter URL generation [\#324](https://github.com/christianhelle/httpgenerator/pull/324) ([christianhelle](https://github.com/christianhelle))
- Fix failing PathLevelParametersTests assertions for OneFile mode [\#323](https://github.com/christianhelle/httpgenerator/pull/323) ([christianhelle](https://github.com/christianhelle))
- Generate meaningful JSON samples for allOf/oneOf/anyOf schemas [\#322](https://github.com/christianhelle/httpgenerator/pull/322) ([christianhelle](https://github.com/christianhelle))
- Correctly append query parameters to URL [\#319](https://github.com/christianhelle/httpgenerator/pull/319) ([christianhelle](https://github.com/christianhelle))
- Merge path-level parameters into operation parameters [\#318](https://github.com/christianhelle/httpgenerator/pull/318) ([christianhelle](https://github.com/christianhelle))
- Null-guard operation.Parameters and parameter entries [\#316](https://github.com/christianhelle/httpgenerator/pull/316) ([christianhelle](https://github.com/christianhelle))
- Fix string handling edge cases and case-insensitive URL/file detection [\#290](https://github.com/christianhelle/httpgenerator/pull/290) ([Copilot](https://github.com/apps/copilot-swe-agent))

**Fixed bugs:**

- Query parameters incorrectly appended for operations with both path and query params [\#315](https://github.com/christianhelle/httpgenerator/issues/315)
- Duplicate filename collisions in OneRequestPerFile mode for large API specs [\#314](https://github.com/christianhelle/httpgenerator/issues/314)
- Path-level parameters not merged into operation parameters [\#312](https://github.com/christianhelle/httpgenerator/issues/312)
- NullReferenceException when Parameters list contains null entries \(unresolved $ef\) [\#311](https://github.com/christianhelle/httpgenerator/issues/311)
- NullReferenceException when operation.Parameters is null \(GitHub API crash\) [\#310](https://github.com/christianhelle/httpgenerator/issues/310)
- Crash and error on int64 properties with minimum limits lower than int32 lowest value [\#36](https://github.com/christianhelle/httpgenerator/issues/36)

**Closed issues:**

- \[deps-010\] Final regression, docs, and release closeout [\#336](https://github.com/christianhelle/httpgenerator/issues/336)
- \[deps-009\] Refresh VSIX SDK packages on 17.x line [\#335](https://github.com/christianhelle/httpgenerator/issues/335)
- \[deps-008\] Refresh OpenAPI regression tests and smoke coverage [\#334](https://github.com/christianhelle/httpgenerator/issues/334)
- \[deps-007\] Reconcile generator and CLI after OpenAPI migration [\#333](https://github.com/christianhelle/httpgenerator/issues/333)
- \[deps-006\] Migrate validator visitor/statistics to OpenAPI v3 [\#332](https://github.com/christianhelle/httpgenerator/issues/332)
- \[deps-005\] Migrate OpenAPI reader pipeline to v3 package set [\#331](https://github.com/christianhelle/httpgenerator/issues/331)
- \[deps-004\] Upgrade Atc.Test while keeping FluentAssertions pinned [\#330](https://github.com/christianhelle/httpgenerator/issues/330)
- \[deps-003\] Upgrade Spectre.Console.Cli [\#329](https://github.com/christianhelle/httpgenerator/issues/329)
- \[deps-002\] Upgrade Microsoft.SourceLink.GitHub [\#328](https://github.com/christianhelle/httpgenerator/issues/328)
- \[deps-001\] Capture baseline and open tracking board [\#327](https://github.com/christianhelle/httpgenerator/issues/327)

**Merged pull requests:**

- chore\(deps\): update dependency oasreader to 3.5.0.19 [\#351](https://github.com/christianhelle/httpgenerator/pull/351) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update microsoft/setup-msbuild action to v3 [\#350](https://github.com/christianhelle/httpgenerator/pull/350) ([renovate[bot]](https://github.com/apps/renovate))
- Regression tests for path-level parameter merging [\#320](https://github.com/christianhelle/httpgenerator/pull/320) ([christianhelle](https://github.com/christianhelle))
- Regression tests for null operation.Parameters [\#317](https://github.com/christianhelle/httpgenerator/pull/317) ([christianhelle](https://github.com/christianhelle))
- chore\(deps\): update actions/github-script action to v8 [\#309](https://github.com/christianhelle/httpgenerator/pull/309) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/checkout action to v6 [\#308](https://github.com/christianhelle/httpgenerator/pull/308) ([renovate[bot]](https://github.com/apps/renovate))
- Setup Squad [\#307](https://github.com/christianhelle/httpgenerator/pull/307) ([christianhelle](https://github.com/christianhelle))
- chore\(deps\): update dependency coverlet.collector to 8.0.1 [\#306](https://github.com/christianhelle/httpgenerator/pull/306) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency ruby to v4.0.2 [\#305](https://github.com/christianhelle/httpgenerator/pull/305) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update ghcr.io/devcontainers/features/powershell docker tag to v2 [\#304](https://github.com/christianhelle/httpgenerator/pull/304) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency coverlet.collector to v8 [\#303](https://github.com/christianhelle/httpgenerator/pull/303) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency microsoft.net.test.sdk to 18.3.0 [\#302](https://github.com/christianhelle/httpgenerator/pull/302) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency community.visualstudio.toolkit.16 to 16.0.549 [\#299](https://github.com/christianhelle/httpgenerator/pull/299) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency system.text.json to 10.0.5 [\#297](https://github.com/christianhelle/httpgenerator/pull/297) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency ruby to v4 [\#296](https://github.com/christianhelle/httpgenerator/pull/296) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/vscode to v1.110.0 [\#293](https://github.com/christianhelle/httpgenerator/pull/293) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v24.12.0 [\#292](https://github.com/christianhelle/httpgenerator/pull/292) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v24.10.2 [\#291](https://github.com/christianhelle/httpgenerator/pull/291) ([renovate[bot]](https://github.com/apps/renovate))
- Revert breaking OpenAPI v3.x upgrades and fix Spectre.Console.Cli signature [\#287](https://github.com/christianhelle/httpgenerator/pull/287) ([Copilot](https://github.com/apps/copilot-swe-agent))
- chore\(deps\): update actions/checkout action to v6 [\#286](https://github.com/christianhelle/httpgenerator/pull/286) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency microsoft.extensions.azure to 1.13.1 [\#285](https://github.com/christianhelle/httpgenerator/pull/285) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/vscode to v1.106.1 [\#284](https://github.com/christianhelle/httpgenerator/pull/284) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v24.10.1 [\#283](https://github.com/christianhelle/httpgenerator/pull/283) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update openapi-dotnet monorepo to v3 \(major\) [\#282](https://github.com/christianhelle/httpgenerator/pull/282) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency system.text.json to v10 [\#281](https://github.com/christianhelle/httpgenerator/pull/281) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency microsoft.net.test.sdk to 18.0.1 [\#279](https://github.com/christianhelle/httpgenerator/pull/279) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency node to v24 [\#278](https://github.com/christianhelle/httpgenerator/pull/278) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency spectre.console.cli to 0.53.0 [\#277](https://github.com/christianhelle/httpgenerator/pull/277) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/upload-artifact action to v5 [\#276](https://github.com/christianhelle/httpgenerator/pull/276) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v22.18.11 [\#273](https://github.com/christianhelle/httpgenerator/pull/273) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/setup-node action to v6 [\#270](https://github.com/christianhelle/httpgenerator/pull/270) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency spectre.console.cli to 0.52.0 [\#269](https://github.com/christianhelle/httpgenerator/pull/269) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/vscode to v1.105.0 [\#268](https://github.com/christianhelle/httpgenerator/pull/268) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v22.18.10 [\#267](https://github.com/christianhelle/httpgenerator/pull/267) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency ruby to v3.4.7 - autoclosed [\#266](https://github.com/christianhelle/httpgenerator/pull/266) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update openapi-dotnet monorepo to 1.6.28 [\#265](https://github.com/christianhelle/httpgenerator/pull/265) ([renovate[bot]](https://github.com/apps/renovate))

## [1.0.1-preview.17](https://github.com/christianhelle/httpgenerator/tree/1.0.1-preview.17) (2025-10-08)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/1.0.0...1.0.1-preview.17)

**Implemented enhancements:**

- Visual Studio Code Extension [\#195](https://github.com/christianhelle/httpgenerator/issues/195)
- Add skip header option [\#253](https://github.com/christianhelle/httpgenerator/pull/253) ([PolarTango](https://github.com/PolarTango))
- Improve code coverage [\#242](https://github.com/christianhelle/httpgenerator/pull/242) ([christianhelle](https://github.com/christianhelle))
- Revert NSwag back to v14.4.0 [\#240](https://github.com/christianhelle/httpgenerator/pull/240) ([christianhelle](https://github.com/christianhelle))
- Fancy CLI output using Spectre Console [\#209](https://github.com/christianhelle/httpgenerator/pull/209) ([christianhelle](https://github.com/christianhelle))
- Visual Studio Code Extension [\#198](https://github.com/christianhelle/httpgenerator/pull/198) ([christianhelle](https://github.com/christianhelle))

**Closed issues:**

- Migrate from using NSwag to Microsoft.OpenApi [\#254](https://github.com/christianhelle/httpgenerator/issues/254)
- Setup CoPilot Instructions [\#231](https://github.com/christianhelle/httpgenerator/issues/231)
- Create Static Documentation Website from README [\#218](https://github.com/christianhelle/httpgenerator/issues/218)
- VSIX build is failing [\#210](https://github.com/christianhelle/httpgenerator/issues/210)

**Merged pull requests:**

- chore\(deps\): update dependency microsoft.net.test.sdk to v18 [\#263](https://github.com/christianhelle/httpgenerator/pull/263) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency spectre.console.cli to 0.51.1 [\#261](https://github.com/christianhelle/httpgenerator/pull/261) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency microsoft.extensions.azure to 1.13.0 [\#259](https://github.com/christianhelle/httpgenerator/pull/259) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update openapi-dotnet monorepo to 1.6.27 [\#258](https://github.com/christianhelle/httpgenerator/pull/258) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency xunit.runner.visualstudio to 3.1.5 [\#257](https://github.com/christianhelle/httpgenerator/pull/257) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency typescript to v5.9.3 [\#256](https://github.com/christianhelle/httpgenerator/pull/256) ([renovate[bot]](https://github.com/apps/renovate))
- Migrate from NSwag to Microsoft.OpenApi libraries with enhanced OpenAPI 3.1 support [\#255](https://github.com/christianhelle/httpgenerator/pull/255) ([Copilot](https://github.com/apps/copilot-swe-agent))
- chore\(deps\): update dependency @types/vscode to v1.104.0 [\#252](https://github.com/christianhelle/httpgenerator/pull/252) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency system.text.json to 9.0.9 [\#251](https://github.com/christianhelle/httpgenerator/pull/251) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/setup-node action to v5 [\#248](https://github.com/christianhelle/httpgenerator/pull/248) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/setup-dotnet action to v5 [\#247](https://github.com/christianhelle/httpgenerator/pull/247) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update actions/github-script action to v8 [\#246](https://github.com/christianhelle/httpgenerator/pull/246) ([renovate[bot]](https://github.com/apps/renovate))
- chore\(deps\): update dependency @types/node to v22.18.8 [\#243](https://github.com/christianhelle/httpgenerator/pull/243) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.18.0 [\#239](https://github.com/christianhelle/httpgenerator/pull/239) ([renovate[bot]](https://github.com/apps/renovate))
- Update actions/upload-pages-artifact action to v4 [\#238](https://github.com/christianhelle/httpgenerator/pull/238) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.Readers to 1.6.25 [\#237](https://github.com/christianhelle/httpgenerator/pull/237) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency xunit.runner.visualstudio to 3.1.4 [\#236](https://github.com/christianhelle/httpgenerator/pull/236) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/vscode to v1.103.0 [\#235](https://github.com/christianhelle/httpgenerator/pull/235) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.17.2 [\#234](https://github.com/christianhelle/httpgenerator/pull/234) ([renovate[bot]](https://github.com/apps/renovate))
- Update actions/checkout action to v5 [\#233](https://github.com/christianhelle/httpgenerator/pull/233) ([renovate[bot]](https://github.com/apps/renovate))
- Create comprehensive GitHub Copilot instructions for httpgenerator development [\#232](https://github.com/christianhelle/httpgenerator/pull/232) ([Copilot](https://github.com/apps/copilot-swe-agent))
- Update nswag monorepo to 14.5.0 [\#230](https://github.com/christianhelle/httpgenerator/pull/230) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.16.5 [\#227](https://github.com/christianhelle/httpgenerator/pull/227) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.16.4 [\#226](https://github.com/christianhelle/httpgenerator/pull/226) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency xunit.runner.visualstudio to 3.1.3 [\#225](https://github.com/christianhelle/httpgenerator/pull/225) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/vscode to v1.102.0 [\#223](https://github.com/christianhelle/httpgenerator/pull/223) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.16.3 [\#222](https://github.com/christianhelle/httpgenerator/pull/222) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency System.Text.Json to 9.0.8 [\#221](https://github.com/christianhelle/httpgenerator/pull/221) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.16.2 [\#220](https://github.com/christianhelle/httpgenerator/pull/220) ([renovate[bot]](https://github.com/apps/renovate))
- Create Static Documentation Website from README [\#219](https://github.com/christianhelle/httpgenerator/pull/219) ([Copilot](https://github.com/apps/copilot-swe-agent))
- Update dependency @types/node to v22.16.0 [\#216](https://github.com/christianhelle/httpgenerator/pull/216) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.15.34 [\#215](https://github.com/christianhelle/httpgenerator/pull/215) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.15.32 [\#214](https://github.com/christianhelle/httpgenerator/pull/214) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.Extensions.Azure to 1.12.0 [\#213](https://github.com/christianhelle/httpgenerator/pull/213) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/vscode to v1.101.0 [\#212](https://github.com/christianhelle/httpgenerator/pull/212) ([renovate[bot]](https://github.com/apps/renovate))
- Fix VSIX build workflows by switching from dotnet restore to msbuild restore [\#211](https://github.com/christianhelle/httpgenerator/pull/211) ([Copilot](https://github.com/apps/copilot-swe-agent))
- Update dependency xunit.runner.visualstudio to 3.1.1 [\#208](https://github.com/christianhelle/httpgenerator/pull/208) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.NET.Test.Sdk to 17.14.1 [\#207](https://github.com/christianhelle/httpgenerator/pull/207) ([renovate[bot]](https://github.com/apps/renovate))
- Create comprehensive contribution guidelines document [\#206](https://github.com/christianhelle/httpgenerator/pull/206) ([Copilot](https://github.com/apps/copilot-swe-agent))
- Update dependency @types/node to v22.15.31 [\#204](https://github.com/christianhelle/httpgenerator/pull/204) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.15.23 [\#203](https://github.com/christianhelle/httpgenerator/pull/203) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency @types/node to v22.15.22 [\#202](https://github.com/christianhelle/httpgenerator/pull/202) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency typescript to v5 [\#201](https://github.com/christianhelle/httpgenerator/pull/201) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency node to v22 [\#200](https://github.com/christianhelle/httpgenerator/pull/200) ([renovate[bot]](https://github.com/apps/renovate))
- Update actions/setup-node action to v4 [\#199](https://github.com/christianhelle/httpgenerator/pull/199) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.NET.Test.Sdk to 17.14.0 [\#197](https://github.com/christianhelle/httpgenerator/pull/197) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency System.Text.Json to 9.0.6 [\#194](https://github.com/christianhelle/httpgenerator/pull/194) ([renovate[bot]](https://github.com/apps/renovate))
- Update nswag monorepo to 14.4.0 [\#193](https://github.com/christianhelle/httpgenerator/pull/193) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency xunit.runner.visualstudio to 3.1.0 [\#176](https://github.com/christianhelle/httpgenerator/pull/176) ([renovate[bot]](https://github.com/apps/renovate))

## [1.0.0](https://github.com/christianhelle/httpgenerator/tree/1.0.0) (2025-04-26)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.7.0...1.0.0)

**Implemented enhancements:**

- Replacing of the current base url [\#158](https://github.com/christianhelle/httpgenerator/issues/158)
- Dev Container [\#192](https://github.com/christianhelle/httpgenerator/pull/192) ([christianhelle](https://github.com/christianhelle))
- feat:Default baseUrl to use for all requests [\#191](https://github.com/christianhelle/httpgenerator/pull/191) ([MrXhh](https://github.com/MrXhh))
- Fix Builds [\#183](https://github.com/christianhelle/httpgenerator/pull/183) ([christianhelle](https://github.com/christianhelle))

**Merged pull requests:**

- Update dependency windows to v2025 [\#190](https://github.com/christianhelle/httpgenerator/pull/190) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Exceptionless to 6.1.0 [\#187](https://github.com/christianhelle/httpgenerator/pull/187) ([renovate[bot]](https://github.com/apps/renovate))
- Update nswag monorepo to 14.3.0 [\#186](https://github.com/christianhelle/httpgenerator/pull/186) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.Readers to 1.6.24 [\#185](https://github.com/christianhelle/httpgenerator/pull/185) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.OData to 1.7.5 [\#184](https://github.com/christianhelle/httpgenerator/pull/184) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.NET.Test.Sdk to 17.13.0 [\#177](https://github.com/christianhelle/httpgenerator/pull/177) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.Extensions.Azure to 1.11.0 [\#175](https://github.com/christianhelle/httpgenerator/pull/175) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.OData to 1.7.1 [\#169](https://github.com/christianhelle/httpgenerator/pull/169) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency System.Text.Json to 9.0.1 [\#167](https://github.com/christianhelle/httpgenerator/pull/167) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.OData to 1.7.0 [\#164](https://github.com/christianhelle/httpgenerator/pull/164) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Atc.Test to 1.1.9 [\#163](https://github.com/christianhelle/httpgenerator/pull/163) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency coverlet.collector to 6.0.3 [\#162](https://github.com/christianhelle/httpgenerator/pull/162) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.OData to 1.6.9 [\#161](https://github.com/christianhelle/httpgenerator/pull/161) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency FluentAssertions to v7 [\#157](https://github.com/christianhelle/httpgenerator/pull/157) ([renovate[bot]](https://github.com/apps/renovate))

## [0.7.0](https://github.com/christianhelle/httpgenerator/tree/0.7.0) (2024-10-30)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.7.0-preview.15...0.7.0)

## [0.7.0-preview.15](https://github.com/christianhelle/httpgenerator/tree/0.7.0-preview.15) (2024-10-29)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.6.0...0.7.0-preview.15)

**Implemented enhancements:**

- Custom header for generated requests [\#148](https://github.com/christianhelle/httpgenerator/issues/148)

## [0.6.0](https://github.com/christianhelle/httpgenerator/tree/0.6.0) (2024-10-13)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.5.0...0.6.0)

**Implemented enhancements:**

- Exceptionless monthly limit exceeded in only a few days [\#134](https://github.com/christianhelle/httpgenerator/issues/134)

## [0.5.0](https://github.com/christianhelle/httpgenerator/tree/0.5.0) (2024-09-20)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.4.0...0.5.0)

**Fixed bugs:**

- Query string parameters are not included in the request URL [\#109](https://github.com/christianhelle/httpgenerator/issues/109)

## [0.4.0](https://github.com/christianhelle/httpgenerator/tree/0.4.0) (2024-06-06)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.3.2...0.4.0)

**Implemented enhancements:**

- Generate one file per tag [\#97](https://github.com/christianhelle/httpgenerator/issues/97)

**Fixed bugs:**

- Multiline XML Comments are not represented ok [\#94](https://github.com/christianhelle/httpgenerator/issues/94)

## [0.3.2](https://github.com/christianhelle/httpgenerator/tree/0.3.2) (2024-04-14)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.3.1...0.3.2)

**Implemented enhancements:**

- Fix issue when setting --base-url to an environment variable [\#71](https://github.com/christianhelle/httpgenerator/pull/71) ([christianhelle](https://github.com/christianhelle))

**Fixed bugs:**

- Does not display error messages [\#78](https://github.com/christianhelle/httpgenerator/issues/78)

**Merged pull requests:**

- Update nswag monorepo to v14.0.7 [\#73](https://github.com/christianhelle/httpgenerator/pull/73) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.OData to v1.6.0 [\#70](https://github.com/christianhelle/httpgenerator/pull/70) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.VSSDK.BuildTools to v17.9.3174 [\#69](https://github.com/christianhelle/httpgenerator/pull/69) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency coverlet.collector to v6.0.2 [\#68](https://github.com/christianhelle/httpgenerator/pull/68) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.Readers to v1.6.14 [\#67](https://github.com/christianhelle/httpgenerator/pull/67) ([renovate[bot]](https://github.com/apps/renovate))

## [0.3.1](https://github.com/christianhelle/httpgenerator/tree/0.3.1) (2024-02-28)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.3.0...0.3.1)

**Implemented enhancements:**

- Update StringExtensions.cs [\#66](https://github.com/christianhelle/httpgenerator/pull/66) ([sjchapmanuk](https://github.com/sjchapmanuk))

**Merged pull requests:**

- Revert "Update actions/upload-artifact action to v4" [\#64](https://github.com/christianhelle/httpgenerator/pull/64) ([christianhelle](https://github.com/christianhelle))
- Update dependency coverlet.collector to v6.0.1 [\#63](https://github.com/christianhelle/httpgenerator/pull/63) ([renovate[bot]](https://github.com/apps/renovate))
- Update actions/upload-artifact action to v4 [\#62](https://github.com/christianhelle/httpgenerator/pull/62) ([renovate[bot]](https://github.com/apps/renovate))
- Update xunit-dotnet monorepo [\#61](https://github.com/christianhelle/httpgenerator/pull/61) ([renovate[bot]](https://github.com/apps/renovate))

## [0.3.0](https://github.com/christianhelle/httpgenerator/tree/0.3.0) (2024-02-09)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.2.7...0.3.0)

**Implemented enhancements:**

- Reference Token from environment file [\#51](https://github.com/christianhelle/httpgenerator/issues/51)
- Add support for loading authorization header from environment variable [\#57](https://github.com/christianhelle/httpgenerator/pull/57) ([christianhelle](https://github.com/christianhelle))
- Prefix route variables with operation name when generating single file [\#56](https://github.com/christianhelle/httpgenerator/pull/56) ([christianhelle](https://github.com/christianhelle))

**Merged pull requests:**

- Update dependency Microsoft.VSSDK.BuildTools to v17.9.3168 [\#58](https://github.com/christianhelle/httpgenerator/pull/58) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.NET.Test.Sdk to v17.9.0 [\#55](https://github.com/christianhelle/httpgenerator/pull/55) ([renovate[bot]](https://github.com/apps/renovate))
- Update nswag monorepo to v14.0.3 [\#54](https://github.com/christianhelle/httpgenerator/pull/54) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency Microsoft.OpenApi.Readers to v1.6.13 [\#50](https://github.com/christianhelle/httpgenerator/pull/50) ([renovate[bot]](https://github.com/apps/renovate))

## [0.2.7](https://github.com/christianhelle/httpgenerator/tree/0.2.7) (2024-01-17)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.1.6...0.2.7)

**Merged pull requests:**

- Update dependency Microsoft.OpenApi.Readers to v1.6.12 [\#47](https://github.com/christianhelle/httpgenerator/pull/47) ([renovate[bot]](https://github.com/apps/renovate))
- Update dependency xunit to v2.6.6 [\#46](https://github.com/christianhelle/httpgenerator/pull/46) ([renovate[bot]](https://github.com/apps/renovate))
- Disable Dependabot [\#43](https://github.com/christianhelle/httpgenerator/pull/43) ([christianhelle](https://github.com/christianhelle))
- Update dependency xunit to v2.6.5 [\#42](https://github.com/christianhelle/httpgenerator/pull/42) ([renovate[bot]](https://github.com/apps/renovate))
- Update actions/checkout action to v4 [\#38](https://github.com/christianhelle/httpgenerator/pull/38) ([renovate[bot]](https://github.com/apps/renovate))
- Update microsoft/setup-msbuild action to v1.3 - autoclosed [\#37](https://github.com/christianhelle/httpgenerator/pull/37) ([renovate[bot]](https://github.com/apps/renovate))
- Update xunit-dotnet monorepo [\#35](https://github.com/christianhelle/httpgenerator/pull/35) ([renovate[bot]](https://github.com/apps/renovate))

## [0.1.6](https://github.com/christianhelle/httpgenerator/tree/0.1.6) (2023-11-29)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.1.5...0.1.6)

## [0.1.5](https://github.com/christianhelle/httpgenerator/tree/0.1.5) (2023-11-23)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.1.4...0.1.5)

**Merged pull requests:**

- Bump Spectre.Console.Cli from 0.47.0 to 0.48.0 [\#9](https://github.com/christianhelle/httpgenerator/pull/9) ([dependabot[bot]](https://github.com/apps/dependabot))
- Bump Microsoft.OpenApi.Readers from 1.6.10 to 1.6.11 [\#8](https://github.com/christianhelle/httpgenerator/pull/8) ([dependabot[bot]](https://github.com/apps/dependabot))

## [0.1.4](https://github.com/christianhelle/httpgenerator/tree/0.1.4) (2023-11-22)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.1.3...0.1.4)

## [0.1.3](https://github.com/christianhelle/httpgenerator/tree/0.1.3) (2023-11-22)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/0.1.1...0.1.3)

## [0.1.1](https://github.com/christianhelle/httpgenerator/tree/0.1.1) (2023-11-15)

[Full Changelog](https://github.com/christianhelle/httpgenerator/compare/bfe3d0ed56ff1e60f124358a17fc44b88c4435e9...0.1.1)



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
