# HTTP File Generator

Generate .http files from OpenAPI specifications

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which then was adopted by JetBrains IDE's, and later on [Visual Studio 2022](https://marketplace.visualstudio.com/items?itemName=MadsKristensen.RestClient)

## Usage

From the **Tools** menu select **Generate .http files**

![Tools menu](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_tools.png?raw=true)

This opens the main dialog which has similar input fields as the CLI tool

![Main dialog](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_httpgenerator_dialog.png?raw=true)

You can acquire an Azure Entra ID access token by clicking on the `...` button beside the **Authorization Headers** input field

![Acquire Azure Entra ID access token](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_azure_entra_id.png?raw=true)

By default, the **Output folder** is pre-filled with the path of the currently active C# Project in the Solution Explorer, suffixed with **\HttpFiles**

![Solution explorer](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_solution_explorer.png?raw=true)

Once the .http files are generated you can easily open and inspect them

![.http file](https://github.com/christianhelle/httpgenerator/blob/main/images/vsix_http_file.png?raw=true)

## Notice

This project is in its very early stages and its marketplace visibility is supposed to be marked as PREVIEW
