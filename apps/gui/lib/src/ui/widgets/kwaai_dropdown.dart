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
/// The popup mirrors macOS system menus: shrink-wrapped to the longest
/// item's width (not the trigger's), tight per-row padding, an instant
/// open/close (no fade), and a checkmark on the currently-selected item.
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
  final _menuController = MenuController();
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

  String? get _selectedLabel {
    for (final item in widget.items) {
      if (item.value == widget.value) return item.label;
    }
    return null;
  }

  int get _selectedIndex {
    for (var i = 0; i < widget.items.length; i++) {
      if (widget.items[i].value == widget.value) return i;
    }
    return -1;
  }

  void _select(T value) {
    widget.onChanged?.call(value);
    _menuController.close();
  }

  /// Measured pixel width of the widest item label using the current
  /// text theme. Used to give the trigger field a fixed width that
  /// matches the popup's intrinsic width — without it, the trigger
  /// either collapses to its content (just the selection) or expands to
  /// the parent's loose constraints.
  double _widestLabelWidth(BuildContext context) {
    final style = Theme.of(context).textTheme.bodyMedium?.copyWith(
      height: 1.0,
    );
    final textScale = MediaQuery.textScalerOf(context);
    double maxW = 0;
    for (final item in widget.items) {
      final tp = TextPainter(
        text: TextSpan(text: item.label, style: style),
        textDirection: TextDirection.ltr,
        textScaler: textScale,
      )..layout();
      if (tp.width > maxW) maxW = tp.width;
      tp.dispose();
    }
    return maxW;
  }

  @override
  Widget build(BuildContext context) {
    final accent = context.kwaai.accentPrimary;
    final scheme = Theme.of(context).colorScheme;
    final radius = BorderRadius.circular(6);
    final fillColor = widget.enabled
        ? context.kwaai.inputBackground
        : Color.alphaBlend(
            scheme.onSurface.withValues(alpha: 0.06),
            context.kwaai.inputBackground,
          );

    // Trigger width = widest label + content padding (10 each side) +
    // chevron column (24) + a small fudge for ellipsis safety.
    final triggerWidth = _widestLabelWidth(context) + 10 + 10 + 24 + 4;

    final restingBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(color: scheme.outline.withValues(alpha: 0.4)),
    );
    final focusedBorder = OutlineInputBorder(
      borderRadius: radius,
      borderSide: BorderSide(color: accent, width: 2),
    );

    // macOS NSPopUpButton-style positioning: open the menu so the
    // currently-selected row sits exactly over the trigger row.
    //
    // With alignment=topStart, the menu's top edge anchors to the
    // trigger's top edge. Trigger row 0 text center is at the same
    // distance from the menu's top as row 0 text center is from the
    // trigger's top (both = menuPad + rowPad + textCenter = 4 + 4 + 6.5
    // = 14.5px). So for index 0 the offset is 0; each additional row
    // shifts the menu up by rowHeight to bring that row's center into
    // alignment with the trigger.
    const rowHeight = 25.0; // 6+6 padding + ~13 text @ height:1.0
    final selectedRowOffset = _selectedIndex < 0
        ? 0.0
        : -(_selectedIndex * rowHeight);

    // Menu surface — uses the theme's menuBackground. The border is a
    // very soft hairline; macOS NSMenu also paints a 1px white inner
    // highlight just inside the border — we approximate that with a
    // Container border on the inner column.
    final menuBg = context.kwaai.menuBackground;
    final menuBorder = scheme.outline.withValues(alpha: 0.25);
    final menuInnerHighlight = Theme.of(context).brightness == Brightness.dark
        ? Colors.white.withValues(alpha: 0.06)
        : Colors.white.withValues(alpha: 0.6);

    return MenuAnchor(
      controller: _menuController,
      childFocusNode: _focusNode,
      alignmentOffset: Offset(0, selectedRowOffset),
      style: MenuStyle(
        alignment: AlignmentDirectional.topStart,
        backgroundColor: WidgetStatePropertyAll(menuBg),
        // Wider, softer drop shadow — Material elevation 12 casts a more
        // diffuse halo than 4–6. The shadow alpha is kept low so the
        // overall effect stays delicate, like macOS NSMenu.
        elevation: const WidgetStatePropertyAll(12),
        shadowColor: WidgetStatePropertyAll(
          Colors.black.withValues(alpha: 0.18),
        ),
        // No outer padding — the inner-highlight Container provides its
        // own breathing room.
        padding: const WidgetStatePropertyAll(EdgeInsets.zero),
        shape: WidgetStatePropertyAll(
          RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(14),
            side: BorderSide(color: menuBorder),
          ),
        ),
      ),
      // Wrap items in an inner Container that paints the white inner
      // highlight (macOS NSMenu has a 1px white edge just inside the
      // border), then in IntrinsicWidth so the menu shrinks to the
      // widest row.
      menuChildren: [
        IntrinsicWidth(
          child: Container(
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(13),
              border: Border.all(color: menuInnerHighlight, width: 1),
            ),
            padding: const EdgeInsets.symmetric(vertical: 4),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              mainAxisSize: MainAxisSize.min,
              children: [
                for (final item in widget.items)
                  _MenuRow<T>(
                    label: item.label,
                    selected: item.value == widget.value,
                    onTap: () => _select(item.value),
                  ),
              ],
            ),
          ),
        ),
      ],
      builder: (context, controller, _) {
        return GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: widget.enabled
              ? () {
                  _focusNode.requestFocus();
                  if (controller.isOpen) {
                    controller.close();
                  } else {
                    controller.open();
                  }
                }
              : null,
          child: Focus(
            focusNode: _focusNode,
            canRequestFocus: widget.enabled,
            // Explicit width = pre-measured widest label + padding/chrome.
            // Without this, the trigger either collapses to the selection's
            // width (visually jumps as the user picks different items) or
            // expands to the parent's loose constraints (full-card width).
            child: SizedBox(
              width: triggerWidth,
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 120),
                curve: Curves.easeOut,
                decoration: BoxDecoration(
                  borderRadius: radius,
                  boxShadow: _focused
                      ? [BoxShadow(color: accent.withValues(alpha: 0.25), blurRadius: 3)]
                      : const [],
                ),
                child: IgnorePointer(
                  child: InputDecorator(
                    isFocused: _focused,
                    isEmpty: _selectedLabel == null,
                    decoration: InputDecoration(
                      hintText: widget.hintText,
                      hintStyle: Theme.of(context).textTheme.bodyMedium
                          ?.copyWith(
                        color: scheme.onSurfaceVariant.withValues(alpha: 0.5),
                      ),
                      isDense: true,
                      filled: true,
                      fillColor: fillColor,
                      hoverColor: Colors.transparent,
                      contentPadding: const EdgeInsets.symmetric(
                        horizontal: 10,
                        vertical: 9,
                      ),
                      border: restingBorder,
                      enabledBorder: restingBorder,
                      disabledBorder: restingBorder,
                      focusedBorder: focusedBorder,
                      suffixIcon: Icon(
                        Icons.unfold_more,
                        size: 16,
                        color: scheme.onSurfaceVariant.withValues(alpha: 0.7),
                      ),
                      suffixIconConstraints: const BoxConstraints(
                        minWidth: 24,
                        minHeight: 24,
                        maxHeight: 24,
                      ),
                    ),
                    child: Text(
                      _selectedLabel ?? '',
                      style: Theme.of(context).textTheme.bodyMedium
                          ?.copyWith(height: 1.0),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}

/// One row in the popup. Tight padding, optional leading checkmark slot,
/// hover/highlight on the row only (not full-bleed).
class _MenuRow<T> extends StatefulWidget {
  const _MenuRow({
    required this.label,
    required this.selected,
    required this.onTap,
  });

  final String label;
  final bool selected;
  final VoidCallback onTap;

  @override
  State<_MenuRow<T>> createState() => _MenuRowState<T>();
}

class _MenuRowState<T> extends State<_MenuRow<T>> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final accent = context.kwaai.accentPrimary;
    final hoverFg = _hovered ? Colors.white : scheme.onSurface;
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: widget.onTap,
        // Outer padding gives the hover pill a left/right gutter inside
        // the menu (macOS NSMenu has this); the inner Container paints
        // the hover fill with a slight corner radius so it reads as a
        // distinct pill rather than full-bleed.
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 4),
          child: Container(
            width: double.infinity,
            decoration: BoxDecoration(
              color: _hovered ? accent : Colors.transparent,
              borderRadius: BorderRadius.circular(6),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 6),
            child: Row(
              children: [
                // Fixed-width checkmark slot keeps labels aligned across rows.
                SizedBox(
                  width: 16,
                  child: widget.selected
                      ? Icon(Icons.check, size: 14, color: hoverFg)
                      : null,
                ),
                const SizedBox(width: 4),
                Text(
                  widget.label,
                  style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    height: 1.0,
                    color: hoverFg,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
