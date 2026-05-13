import 'package:flutter_test/flutter_test.dart';

import 'package:kwaainet_gui/main.dart';

void main() {
  testWidgets('renders scaffold landing screen', (WidgetTester tester) async {
    await tester.pumpWidget(const KwaainetGuiApp());

    expect(find.text('kwaainet-gui'), findsOneWidget);
    expect(find.text('KwaaiNet inference UI — scaffold'), findsOneWidget);
  });
}
