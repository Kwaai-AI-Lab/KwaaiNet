import 'package:flutter/material.dart';

void main() {
  runApp(const KwaainetGuiApp());
}

class KwaainetGuiApp extends StatelessWidget {
  const KwaainetGuiApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'kwaainet-gui',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.indigo),
        useMaterial3: true,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: const Text('kwaainet-gui'),
      ),
      body: const Center(
        child: Text('KwaaiNet inference UI — scaffold'),
      ),
    );
  }
}
