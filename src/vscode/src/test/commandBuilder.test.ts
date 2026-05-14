import { test } from 'node:test';
import * as assert from 'node:assert/strict';

import { createHttpGeneratorCommandForShell } from '../commandBuilder';

test('powershell command uses call operator and handles spaces', () => {
    const command = createHttpGeneratorCommandForShell(
        'C:\\Program Files\\httpgenerator\\httpgenerator.exe',
        'C:\\Users\\chris\\My APIs\\openapi.yaml',
        'C:\\Users\\chris\\Generated Http Files',
        'OneFile',
        'powershell'
    );

    assert.equal(
        command,
        '& \'C:\\Program Files\\httpgenerator\\httpgenerator.exe\' \'C:\\Users\\chris\\My APIs\\openapi.yaml\' --output \'C:\\Users\\chris\\Generated Http Files\' --output-type \'OneFile\''
    );
});

test('cmd command quotes every path argument', () => {
    const command = createHttpGeneratorCommandForShell(
        'C:\\Program Files\\httpgenerator\\httpgenerator.exe',
        'C:\\Users\\chris\\My APIs\\openapi.yaml',
        'C:\\Users\\chris\\Generated Http Files',
        'OneRequestPerFile',
        'cmd'
    );

    assert.equal(
        command,
        '"C:\\Program Files\\httpgenerator\\httpgenerator.exe" "C:\\Users\\chris\\My APIs\\openapi.yaml" --output "C:\\Users\\chris\\Generated Http Files" --output-type "OneRequestPerFile"'
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

test('powershell escapes single quotes in path values', () => {
    const command = createHttpGeneratorCommandForShell(
        "C:\\Tools\\O'Brien\\httpgenerator.exe",
        "C:\\Specs\\O'Brien\\openapi.yaml",
        "C:\\Out\\O'Brien",
        'OneFile',
        'powershell'
    );

    assert.equal(
        command,
        "& 'C:\\Tools\\O''Brien\\httpgenerator.exe' 'C:\\Specs\\O''Brien\\openapi.yaml' --output 'C:\\Out\\O''Brien' --output-type 'OneFile'"
    );
});
