/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import { ExtensionContext } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
	const cargoBuildType = 'debug';
	// The server is implemented in node
	const serverCommand = context.asAbsolutePath(
		path.join('server', 'target', cargoBuildType, 'server')
	);

	const debugOptions = { env: { RUST_LOG: "trace" } };

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run: { command: serverCommand },
		debug: {
			command: serverCommand,
			args: [],
			options: debugOptions
		}
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'bsv' }]
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'blang',
		'B-Lang',
		serverOptions,
		clientOptions,
		true // forceDebug
	);

	// Start the client. This will also launch the server
	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
