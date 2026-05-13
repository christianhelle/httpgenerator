import * as vscode from 'vscode';

export function showProgress<T>(
    title: string,
    step: (progress: vscode.Progress<{ message?: string; increment?: number }>) => Thenable<T>
): Promise<T> {
    return Promise.resolve(vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            title,
            cancellable: false
        },
        step
    ));
}
