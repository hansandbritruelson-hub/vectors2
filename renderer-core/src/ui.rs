use crate::FlexEngine;

pub fn build_ui(engine: &mut FlexEngine) {
    // 1. Setup Scene
    // Grand Root (Index 0) - Row Direction (Sidebar + Main Content)
    let grand_root = engine.add_node(800.0);
    engine.set_flex_direction(grand_root, 0); // 0 = Row
    engine.set_color(grand_root, 0.1, 0.1, 0.1, 1.0); // Dark Background

    // Sidebar (Index 1) - Left Column
    let sidebar = engine.add_node(0.0);
    engine.set_fixed_width(sidebar, 200.0); // Increased width
    engine.set_text(sidebar, "SIDEBAR\n\nDashboard\nAnalytics\nCustomers\nSettings\n\nStatus: OK");
    engine.set_parent(sidebar, grand_root);
    engine.set_color(sidebar, 0.15, 0.15, 0.2, 1.0); // Dark Blue-Gray

    // Main Content (Index 2) - Right Column (Column Direction)
    let main_content = engine.add_node(0.0);
    engine.set_flex_direction(main_content, 1); // 1 = Column
    engine.set_parent(main_content, grand_root);
    engine.set_color(main_content, 0.9, 0.9, 0.9, 1.0); // Light Gray

    // Set Grand Root Children (Sidebar, Main Content)
    engine.set_child_start(grand_root, 1);

    // --- Content Area Below ---

    // Row 1 (Index 3) - Row Direction
    let row1 = engine.add_node(0.0);
    engine.set_parent(row1, main_content);
    engine.set_color(row1, 0.3, 0.3, 0.3, 1.0); // Dark Grey

    // Row 2 (Index 4) - Row Direction
    let row2 = engine.add_node(0.0);
    engine.set_parent(row2, main_content);
    engine.set_color(row2, 0.3, 0.3, 0.3, 1.0); // Dark Grey
    
    // Absolute Item (Index 5)
    let abs_item = engine.add_node(0.0);
    engine.set_text(abs_item, "ABSOLUTE\nPOPUP");
    engine.set_position_absolute(abs_item, 50.0, 50.0); // Top 50, Left 50 relative to Main Content
    engine.set_parent(abs_item, main_content);
    engine.set_color(abs_item, 1.0, 0.2, 0.2, 1.0); // Red
    engine.set_fixed_width(abs_item, 100.0);

    // Set mainContent Children (Row 1, Row 2, Abs Item)
    // Note: Children must be contiguous indices [3, 4, 5]
    engine.set_child_start(main_content, 3); // Children start at index 3

    // Content for Row 1 (Indices 6, 7)
    let t1 = engine.add_node(0.0);
    engine.set_text(t1, "Row 1 - Item A: This is a much longer sentence designed to test the wrapping capabilities of our GPU renderer. It should span multiple lines if everything is working correctly.");
    engine.set_parent(t1, row1);
    
    let t2 = engine.add_node(0.0);
    engine.set_text(t2, "Row 1 - Item B: This is also a significant amount of text to ensure that we have proper distribution of space between these two items in the first row.");
    engine.set_parent(t2, row1);

    engine.set_child_start(row1, 6); // Children start at index 6

    // Content for Row 2 (Indices 8, 9)
    let t3 = engine.add_node(0.0);
    engine.set_text(t3, "Row 2 - Item C: This third block of text is in the second row, which should appear below the first row. It also needs to be long enough to wrap.");
    engine.set_parent(t3, row2);

    let t4 = engine.add_node(0.0);
    engine.set_text(t4, "Row 2 - Item D: Finally, this is the last block of text. By making all of these sentences longer, we stress test the line breaking algorithms in the compute shader.");
    engine.set_parent(t4, row2);

    engine.set_child_start(row2, 8); // Children start at index 8
}
