import { test } from 'node:test';
import * as assert from 'node:assert/strict';

import { createHttpGeneratorCommandForShell } from '../commandBuilder';

test('powershell command uses call operator and handles spaces', () => {
    const command = createHttpGeneratorCommandForShell(
        './httpgenerator/httpgenerator.exe',
        './apis/My APIs/openapi.yaml',
        './output/Generated Http Files',
        'OneFile',
        'powershell'
    );

    assert.equal(
        command,
        "& './httpgenerator/httpgenerator.exe' './apis/My APIs/openapi.yaml' --output './output/Generated Http Files' --output-type 'OneFile'"
    );
});

test('cmd command quotes every path argument', () => {
    const command = createHttpGeneratorCommandForShell(
        './httpgenerator/httpgenerator.exe',
        './apis/My APIs/openapi.yaml',
        './output/Generated Http Files',
        'OneRequestPerFile',
        'cmd'
    );

    assert.equal(
        command,
        '"./httpgenerator/httpgenerator.exe" "./apis/My APIs/openapi.yaml" --output "./output/Generated Http Files" --output-type "OneRequestPerFile"'
    );
});

test('posix command single-quotes every argument', () => {
    const command = createHttpGeneratorCommandForShell(
        '/opt/httpgenerator/bin/httpgenerator',
        '/home/chris/my apis/openapi.yaml',
        '/home/chris/generated http files',
        'OneFile',
        'posix'
    );

    assert.equal(
        command,
        '\'/opt/httpgenerator/bin/httpgenerator\' \'/home/chris/my apis/openapi.yaml\' --output \'/home/chris/generated http files\' --output-type \'OneFile\''
    );
});

test('cmd escapes embedded double-quotes with doubled quotes', () => {
    const command = createHttpGeneratorCommandForShell(
        './tools/"MyTool"/httpgenerator.exe',
        './specs/openapi.yaml',
        './output/normal',
        'OneFile',
        'cmd'
    );

    assert.equal(
        command,
        '"./tools/""MyTool""/httpgenerator.exe" "./specs/openapi.yaml" --output "./output/normal" --output-type "OneFile"'
    );
});

test('powershell escapes single quotes in path values', () => {
    const command = createHttpGeneratorCommandForShell(
        "./tools/O'Brien/httpgenerator.exe",
        "./specs/O'Brien/openapi.yaml",
        "./output/O'Brien",
        'OneFile',
        'powershell'
    );

    assert.equal(
        command,
        "& './tools/O''Brien/httpgenerator.exe' './specs/O''Brien/openapi.yaml' --output './output/O''Brien' --output-type 'OneFile'"
    );
});
