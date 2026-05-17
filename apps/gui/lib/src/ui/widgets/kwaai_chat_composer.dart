import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../theme/kwaai_theme.dart';

/// Multi-line chat composer. Distinct primitive from [KwaaiTextField]:
/// pill-rounded, grows upward as the user types or wraps, no
/// InputDecorator chrome (so Material's `hoverColor` doesn't grey
/// the fill), and an inline circular Send affordance.
///
/// Keyboard:
///   Enter             — send (if [onSend] is non-null)
///   Shift-Enter       — newline
///   Cmd / Ctrl-Enter  — send (kept for muscle memory)
///
/// State:
///   [enabled]     — pure visual / interaction gate. When false the
///                   field doesn't accept input and Send is muted.
///   [onSend]      — non-null to enable Send; null disables it even
///                   when the field accepts text (e.g. mid-stream).
///
/// The widget owns no draft state — the caller's [controller] /
/// [focusNode] are the source of truth, so the parent can clear /
/// refocus from outside (e.g. clear on submit, refocus after stream
/// completion).
class KwaaiChatComposer extends StatelessWidget {
  const KwaaiChatComposer({
    super.key,
    required this.controller,
    required this.focusNode,
    required this.enabled,
    required this.onSend,
    this.onNewChat,
    this.canNewChat = true,
    this.hintText,
    this.minLines = 1,
    this.maxLines = 6,
  });

  final TextEditingController controller;
  final FocusNode focusNode;
  final bool enabled;

  /// Tap target for Send. Null disables Send (mid-stream, daemon
  /// down, empty input — caller's choice). When non-null and the
  /// user hits Cmd/Ctrl-Enter, this fires too.
  final VoidCallback? onSend;

  /// Optional "New chat" affordance, rendered to the right of Send.
  /// Hidden entirely when null so the composer collapses back to
  /// just input + send for callers that don't track transcripts.
  final VoidCallback? onNewChat;

  /// Gates the New Chat button without hiding it — set to false when
  /// the transcript is empty so the button stays discoverable but
  /// reads as inert. Ignored when [onNewChat] is null (button is
  /// already hidden in that case).
  final bool canNewChat;

  final String? hintText;

  /// Initial visual rows; the composer never renders shorter than
  /// this. Defaults to 1 so it looks like a single-line input until
  /// the user wraps.
  final int minLines;

  /// Cap on growth. Beyond this the composer scrolls internally
  /// rather than pushing the surrounding layout further upward.
  /// Defaults to 6 — enough for a reasonable prompt without
  /// crowding the transcript.
  final int maxLines;

  bool get _canSend => enabled && onSend != null;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final ext = context.kwaai;

    // Background tint. Same in idle/hover/focus — Material's default
    // InputDecorator hoverColor would grey the fill on mouse-over,
    // which reads as "this control is half-disabled". We're using a
    // bare Container so the only state-driven change is the disabled
    // wash below.
    final fill = enabled
        ? ext.inputBackground
        : Color.alphaBlend(
            scheme.onSurface.withValues(alpha: 0.06),
            ext.inputBackground,
          );

    return Container(
      decoration: BoxDecoration(
        color: fill,
        borderRadius: BorderRadius.circular(20),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 6),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Expanded(
            child: _ComposerField(
              controller: controller,
              focusNode: focusNode,
              enabled: enabled,
              onSend: _canSend ? onSend : null,
              hintText: hintText,
              minLines: minLines,
              maxLines: maxLines,
            ),
          ),
          const SizedBox(width: 6),
          _SendButton(
            onPressed: _canSend ? onSend : null,
            accent: ext.accentPrimary,
            mutedFg: scheme.onSurfaceVariant.withValues(alpha: 0.5),
          ),
          if (onNewChat != null) ...[
            const SizedBox(width: 4),
            _NewChatButton(
              onPressed: canNewChat ? onNewChat : null,
              fg: scheme.onSurfaceVariant,
            ),
          ],
        ],
      ),
    );
  }
}

/// Borderless icon button that clears the transcript / starts a fresh
/// conversation. Sized to match [_SendButton] so the trailing edge of
/// the composer reads as a coherent action group.
class _NewChatButton extends StatelessWidget {
  const _NewChatButton({required this.onPressed, required this.fg});

  final VoidCallback? onPressed;
  final Color fg;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      tooltip: 'New chat',
      onPressed: onPressed,
      style: IconButton.styleFrom(
        foregroundColor: fg,
        minimumSize: const Size(32, 32),
        padding: EdgeInsets.zero,
        shape: const CircleBorder(),
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      ),
      icon: const Icon(Icons.add_comment_outlined, size: 18),
    );
  }
}

/// The actual text field. Pulled out so [KwaaiChatComposer] can keep
/// its build readable; this is where the multiline + cmd-enter
/// plumbing lives.
class _ComposerField extends StatelessWidget {
  const _ComposerField({
    required this.controller,
    required this.focusNode,
    required this.enabled,
    required this.onSend,
    required this.hintText,
    required this.minLines,
    required this.maxLines,
  });

  final TextEditingController controller;
  final FocusNode focusNode;
  final bool enabled;
  final VoidCallback? onSend;
  final String? hintText;
  final int minLines;
  final int maxLines;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final hintStyle = Theme.of(context).textTheme.bodyMedium?.copyWith(
      color: scheme.onSurfaceVariant.withValues(alpha: 0.55),
    );

    // Plain Enter sends; Shift-Enter inserts a newline (handled by
    // the TextField's default newline action since we don't bind it
    // here). Cmd/Ctrl-Enter also send — kept for muscle memory from
    // the prior binding.
    final shortcuts = <ShortcutActivator, Intent>{
      const SingleActivator(LogicalKeyboardKey.enter): const _SendIntent(),
      const SingleActivator(LogicalKeyboardKey.enter, meta: true):
          const _SendIntent(),
      const SingleActivator(LogicalKeyboardKey.enter, control: true):
          const _SendIntent(),
    };

    return Shortcuts(
      shortcuts: shortcuts,
      child: Actions(
        actions: <Type, Action<Intent>>{
          _SendIntent: CallbackAction<_SendIntent>(
            onInvoke: (_) {
              final fn = onSend;
              if (fn != null) fn();
              return null;
            },
          ),
        },
        child: TextField(
          controller: controller,
          focusNode: focusNode,
          enabled: enabled,
          keyboardType: TextInputType.multiline,
          textInputAction: TextInputAction.newline,
          minLines: minLines,
          maxLines: maxLines,
          style: Theme.of(context).textTheme.bodyMedium,
          decoration: InputDecoration(
            isCollapsed: true,
            contentPadding: const EdgeInsets.symmetric(vertical: 10),
            border: InputBorder.none,
            focusedBorder: InputBorder.none,
            enabledBorder: InputBorder.none,
            disabledBorder: InputBorder.none,
            hintText: hintText,
            hintStyle: hintStyle,
            // Material would otherwise tint the field's *parent*
            // Material widget on hover; with isCollapsed + no border
            // we don't see that tint, but be explicit anyway.
            hoverColor: Colors.transparent,
            filled: false,
          ),
        ),
      ),
    );
  }
}

class _SendIntent extends Intent {
  const _SendIntent();
}

/// Circular Send button. Accent fill when active, muted when not.
class _SendButton extends StatelessWidget {
  const _SendButton({
    required this.onPressed,
    required this.accent,
    required this.mutedFg,
  });

  final VoidCallback? onPressed;
  final Color accent;
  final Color mutedFg;

  @override
  Widget build(BuildContext context) {
    final active = onPressed != null;
    return IconButton(
      tooltip: 'Send (Enter — Shift+Enter for newline)',
      onPressed: onPressed,
      style: IconButton.styleFrom(
        backgroundColor: active ? accent : Colors.transparent,
        foregroundColor: active ? Colors.white : mutedFg,
        minimumSize: const Size(32, 32),
        padding: EdgeInsets.zero,
        shape: const CircleBorder(),
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
      ),
      icon: const Icon(Icons.arrow_upward, size: 18),
    );
  }
}
