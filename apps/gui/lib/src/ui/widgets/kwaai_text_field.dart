import 'package:flutter/material.dart';

import '../theme/kwaai_theme.dart';

/// Themed single-line text input modelled on a native macOS `NSTextField`:
/// tight height, a hairline gray border at rest, and a thin accent focus
/// ring with a subtle glow. Pass [onSubmitted] / [onEditingComplete] as
/// needed.
class KwaaiTextField extends StatefulWidget {
  const KwaaiTextField({
    super.key,
    required this.controller,
    this.hintText,
    this.onSubmitted,
    this.onEditingComplete,
    this.trailing,
    this.enabled = true,
  });

  final TextEditingController controller;
  final String? hintText;
  final ValueChanged<String>? onSubmitted;
  final VoidCallback? onEditingComplete;

  /// Optional control rendered immediately to the right of the input box
  /// (e.g. a Browse… button). Lives in its own layout cell, not inside
  /// the InputDecorator — that keeps the input's own height/centering
  /// invariant to whatever the trailing widget is, and the trailing
  /// widget's hover/tap target behaves naturally at any text scale.
  final Widget? trailing;

  final bool enabled;

  @override
  State<KwaaiTextField> createState() => _KwaaiTextFieldState();
}

class _KwaaiTextFieldState extends State<KwaaiTextField> {
  final _focusNode = FocusNode();
  bool _focused = false;

  @override
  void initState() {
    super.initState();
    _focusNode.addListener(_onFocusChange);
  }

  void _onFocusChange() {
    if (_focusNode.hasFocus != _focused) {
      setState(() => _focused = _focusNode.hasFocus);
    }
  }

  @override
  void dispose() {
    _focusNode.removeListener(_onFocusChange);
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final accent = context.kwaai.accentPrimary;
    final radius = BorderRadius.circular(6);

    // Hairline resting border — a faint neutral gray, not the saturated
    // theme divider colour.
    final restingBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(
        color: Theme.of(context).colorScheme.outline.withValues(alpha: 0.4),
      ),
    );

    // Focus: a thin 2px accent ring, like the macOS focus ring.
    final focusedBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(color: accent, width: 2),
    );

    final field = AnimatedContainer(
      duration: const Duration(milliseconds: 120),
      curve: Curves.easeOut,
      decoration: BoxDecoration(
        borderRadius: radius,
        boxShadow: _focused
            ? [BoxShadow(color: accent.withValues(alpha: 0.25), blurRadius: 3)]
            : const [],
      ),
      child: TextField(
        controller: widget.controller,
        focusNode: _focusNode,
        enabled: widget.enabled,
        style: Theme.of(context).textTheme.bodyMedium?.copyWith(height: 1.0),
        onSubmitted: widget.onSubmitted,
        onEditingComplete: widget.onEditingComplete,
        decoration: InputDecoration(
          hintText: widget.hintText,
          hintStyle: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(
              context,
            ).colorScheme.onSurfaceVariant.withValues(alpha: 0.5),
          ),
          isDense: true,
          filled: true,
          fillColor: widget.enabled
              ? context.kwaai.inputBackground
              : Color.alphaBlend(
                  Theme.of(
                    context,
                  ).colorScheme.onSurface.withValues(alpha: 0.06),
                  context.kwaai.inputBackground,
                ),
          hoverColor: Colors.transparent,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 10,
            vertical: 9,
          ),
          border: restingBorder,
          enabledBorder: restingBorder,
          disabledBorder: restingBorder,
          focusedBorder: focusedBorder,
        ),
      ),
    );

    if (widget.trailing == null) return field;

    // Trailing widget (Browse…, clear, etc.) lives in a sibling cell so
    // it gets its own clean layout and hover behaviour — InputDecorator's
    // suffix slot anchors to the text baseline, which made centering an
    // IconButton brittle across text scales.
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Expanded(child: field),
        const SizedBox(width: 2),
        widget.trailing!,
      ],
    );
  }
}
