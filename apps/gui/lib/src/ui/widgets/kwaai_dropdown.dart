import 'package:flutter/material.dart';

import '../theme/kwaai_theme.dart';

/// One choice in a [KwaaiDropdown].
class KwaaiDropdownItem<T> {
  const KwaaiDropdownItem({required this.value, required this.label});
  final T value;
  final String label;
}

/// Themed single-select dropdown matching [KwaaiTextField]'s look —
/// hairline gray border at rest, an accent focus ring + soft glow on focus,
/// a slightly darker fill when disabled.
///
/// Pass [items] as the list of choices and [value] as the currently
/// selected one (null shows [hintText]). Selection events go to [onChanged].
class KwaaiDropdown<T> extends StatefulWidget {
  const KwaaiDropdown({
    super.key,
    required this.items,
    required this.value,
    required this.onChanged,
    this.hintText,
    this.enabled = true,
  });

  final List<KwaaiDropdownItem<T>> items;
  final T? value;
  final ValueChanged<T?>? onChanged;
  final String? hintText;
  final bool enabled;

  @override
  State<KwaaiDropdown<T>> createState() => _KwaaiDropdownState<T>();
}

class _KwaaiDropdownState<T> extends State<KwaaiDropdown<T>> {
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

    final restingBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(
        color: Theme.of(context).colorScheme.outline.withValues(alpha: 0.4),
      ),
    );

    final focusedBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(color: accent, width: 2),
    );

    return AnimatedContainer(
      duration: const Duration(milliseconds: 120),
      curve: Curves.easeOut,
      decoration: BoxDecoration(
        borderRadius: radius,
        boxShadow: _focused
            ? [BoxShadow(color: accent.withValues(alpha: 0.25), blurRadius: 3)]
            : const [],
      ),
      child: DropdownButtonFormField<T>(
        focusNode: _focusNode,
        initialValue: widget.value,
        onChanged: widget.enabled ? widget.onChanged : null,
        style: Theme.of(context).textTheme.bodyMedium?.copyWith(height: 1.0),
        isDense: true,
        icon: Icon(
          Icons.unfold_more,
          size: 16,
          color: Theme.of(
            context,
          ).colorScheme.onSurfaceVariant.withValues(alpha: 0.7),
        ),
        items: [
          for (final item in widget.items)
            DropdownMenuItem<T>(
              value: item.value,
              child: Text(item.label),
            ),
        ],
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
            vertical: 7,
          ),
          border: restingBorder,
          enabledBorder: restingBorder,
          disabledBorder: restingBorder,
          focusedBorder: focusedBorder,
        ),
      ),
    );
  }
}
