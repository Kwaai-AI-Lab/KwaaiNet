import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:kwaainet_gui/src/chat/chat_state.dart';
import 'package:kwaainet_gui/src/chat/kwaai_rpc_client.dart';

/// Test-only client that emits a fixed list of tokens with no delay.
class _StubClient extends KwaaiRpcClient {
  _StubClient(this.tokens);

  final List<String> tokens;

  @override
  Stream<String> chatStream(String prompt) async* {
    for (final t in tokens) {
      yield t;
    }
  }
}

/// Test-only client that throws mid-stream so we can verify error
/// handling preserves the assistant message.
class _ThrowingClient extends KwaaiRpcClient {
  @override
  Stream<String> chatStream(String prompt) async* {
    yield 'first ';
    throw StateError('boom');
  }
}

void main() {
  test('send appends user + assistant, streams tokens, clears streaming on done',
      () async {
    final container = ProviderContainer(
      overrides: [
        kwaaiRpcClientProvider.overrideWithValue(
          _StubClient(const ['hello ', 'there']),
        ),
      ],
    );
    addTearDown(container.dispose);

    await container.read(chatTranscriptProvider.notifier).send('hi');

    final msgs = container.read(chatTranscriptProvider);
    expect(msgs, hasLength(2));
    expect(msgs[0].role, 'user');
    expect(msgs[0].text, 'hi');
    expect(msgs[1].role, 'assistant');
    expect(msgs[1].text, 'hello there');
    expect(msgs[1].streaming, false);
    expect(container.read(chatStreamingProvider), false);
  });

  test('streaming is true while tokens are arriving', () async {
    final controller = StreamController<String>();
    final container = ProviderContainer(
      overrides: [
        kwaaiRpcClientProvider.overrideWithValue(
          _ControlledClient(controller.stream),
        ),
      ],
    );
    addTearDown(container.dispose);

    final send = container.read(chatTranscriptProvider.notifier).send('hi');
    // Yield once so the StreamSubscription is wired up.
    await Future<void>.delayed(Duration.zero);
    expect(container.read(chatStreamingProvider), true);

    controller.add('a ');
    await Future<void>.delayed(Duration.zero);
    expect(container.read(chatTranscriptProvider).last.text, 'a ');

    controller.add('b');
    await controller.close();
    await send;

    expect(container.read(chatTranscriptProvider).last.text, 'a b');
    expect(container.read(chatStreamingProvider), false);
  });

  test('empty prompt is a no-op', () async {
    final container = ProviderContainer(
      overrides: [
        kwaaiRpcClientProvider.overrideWithValue(_StubClient(const ['x'])),
      ],
    );
    addTearDown(container.dispose);

    await container.read(chatTranscriptProvider.notifier).send('   ');
    expect(container.read(chatTranscriptProvider), isEmpty);
  });

  test('stream error surfaces on assistant message and clears streaming',
      () async {
    final container = ProviderContainer(
      overrides: [
        kwaaiRpcClientProvider.overrideWithValue(_ThrowingClient()),
      ],
    );
    addTearDown(container.dispose);

    await container.read(chatTranscriptProvider.notifier).send('hi');
    final last = container.read(chatTranscriptProvider).last;
    expect(last.role, 'assistant');
    // Tokens that arrived before the error are preserved verbatim;
    // the error lands on the message's `error` field for the UI to
    // render in its distinct red badge.
    expect(last.text, 'first ');
    expect(last.error, contains('boom'));
    expect(last.streaming, false);
  });
}

class _ControlledClient extends KwaaiRpcClient {
  _ControlledClient(this.source);

  final Stream<String> source;

  @override
  Stream<String> chatStream(String prompt) => source;
}
