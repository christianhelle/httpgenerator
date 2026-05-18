# HTTP File Generator

Generate .http files from OpenAPI specifications

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which then was adopted by JetBrains IDE's, and later on [Visual Studio 2022](https://marketplace.visualstudio.com/items?itemName=MadsKristensen.RestClient)

## Usage

Right-click an OpenAPI `.json`, `.yaml`, or `.yml` file in **Solution Explorer** and choose **Generate .http files**.

The **Tools** menu keeps the same command as a fallback, but it only runs when the current Solution Explorer selection is a supported OpenAPI file.

Generation continues in the background after the command starts. Visual Studio stays responsive, progress is reported through the shell progress UI, and cancellation is available from the running progress item.

Non-blocking prompts handle the result:

- **Open Folder** after success
- **Open Activity** when the same spec is already generating
- **View Details** after failures

![Solution explorer](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_solution_explorer.png?raw=true)

Use **Tools** → **HTTP File Generator (PREVIEW)** → **Generation settings and activity** to open the non-blocking tool window. It lets you edit global defaults for:

- sibling `HttpFiles` output folder policy
- base URL override
- content type override
- one-file-per-request generation

The same tool window also shows the latest activity details for queued, running, cancelled, successful, and failed background runs.

By default, generated files go to a sibling `HttpFiles` folder beside the selected OpenAPI document. You can switch the output policy to the spec's own folder in the settings tool window.

The extension resolves `httpgenerator.exe` in this order: `HTTPGENERATOR_PATH`, the bundled VSIX payload, repo-root workspace `target\debug` / `target\release` outputs during development, then `PATH`. If an explicit path is invalid or no candidate is found, generation fails fast.

This first implementation slice does not expose persisted authorization header or Azure Entra ID settings in the Visual Studio UI.

Once the .http files are generated you can easily open and inspect them

![.http file](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_http_file.png?raw=true)

## Notice

This project is in its very early stages and its marketplace visibility is supposed to be marked as PREVIEW
