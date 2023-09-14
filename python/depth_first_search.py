def test_depth_first_search():
    # Test Case 1: A simple linear graph
    graph1 = {'A': ['B'], 'B': ['C'], 'C': []}
    assert depth_first_search(graph1, 'A') == ['A', 'B', 'C']

    # Test Case 2: A disconnected graph
    graph2 = {'A': ['B', 'C'], 'B': ['A'], 'C': ['A'], 'D': []}
    assert depth_first_search(graph2, 'A') == ['A', 'B', 'C']
    assert depth_first_search(graph2, 'D') == ['D']

    # Test Case 3: A graph with loops
    graph3 = {'A': ['B', 'C'], 'B': ['A', 'D', 'E'], 'C': ['A', 'F'], 'D': ['B'], 'E': ['B', 'F'], 'F': ['C', 'E']}
    assert depth_first_search(graph3, 'A') == ['A', 'B', 'D', 'E', 'F', 'C']

    # Test Case 4: Empty graph
    graph4 = {}
    assert depth_first_search(graph4, 'A') == ['A']  # Should return just the start_node

    print("All test cases passed!")


def depth_first_search(graph, start_node):
    visited = []
    stack = [start_node]

    while stack:
        v = stack.pop()
        if v not in visited:
            visited.append(v)
        neighs = graph[v]
        for n in neighs:
            if n not in visited:
                stack.append(n)

    return visited


if __name__ == "__main__":
    test_depth_first_search()
