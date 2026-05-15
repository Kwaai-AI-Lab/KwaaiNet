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
    this.suffixIcon,
    this.enabled = true,
  });

  final TextEditingController controller;
  final String? hintText;
  final ValueChanged<String>? onSubmitted;
  final VoidCallback? onEditingComplete;
  final Widget? suffixIcon;
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

    return AnimatedContainer(
      duration: const Duration(milliseconds: 120),
      curve: Curves.easeOut,
      decoration: BoxDecoration(
        borderRadius: radius,
        // Subtle focus glow — soft, low-spread, not a fat halo.
        boxShadow: _focused
            ? [BoxShadow(color: accent.withValues(alpha: 0.25), blurRadius: 3)]
            : const [],
      ),
      child: TextField(
        controller: widget.controller,
        focusNode: _focusNode,
        enabled: widget.enabled,
        // Tight line height so the field height is driven by the text cap
        // height + contentPadding, not Material's tall default line box.
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
          // Same fill across enabled / focused / hover states (no Material
          // focus tint shift). Disabled state gets a slightly darker fill
          // so it reads as inactive.
          fillColor: widget.enabled
              ? context.kwaai.inputBackground
              : Color.alphaBlend(
                  Theme.of(
                    context,
                  ).colorScheme.onSurface.withValues(alpha: 0.06),
                  context.kwaai.inputBackground,
                ),
          hoverColor: Colors.transparent,
          // Tight, macOS-like field height. With the 1.0 line height above,
          // this padding is the field's only vertical breathing room.
          // Inside-the-border padding gives the text some breathing room
          // matching the radio rows' visual rhythm.
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 10,
            vertical: 7,
          ),
          border: restingBorder,
          enabledBorder: restingBorder,
          disabledBorder: restingBorder,
          focusedBorder: focusedBorder,
          suffixIcon: widget.suffixIcon,
          // Keep a suffix IconButton from forcing the 48px Material tap
          // target, which would make the field tall again.
          suffixIconConstraints: const BoxConstraints(
            minWidth: 30,
            minHeight: 30,
          ),
        ),
      ),
    );
  }
}
