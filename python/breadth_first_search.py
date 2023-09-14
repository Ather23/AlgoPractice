def test_breadth_first_search():
    # Test 1: Simple connected graph
    graph1 = {'A': ['B', 'C'], 'B': ['A', 'D'], 'C': ['A', 'D'], 'D': ['B', 'C']}
    assert bfs_function(graph1, 'A') in [['A', 'B', 'C', 'D'], ['A', 'C', 'B', 'D']]

    # Test 2: Disconnected graph
    graph2 = {'A': ['B'], 'B': ['A'], 'C': ['D'], 'D': ['C']}
    assert bfs_function(graph2, 'A') == ['A', 'B']
    assert bfs_function(graph2, 'C') == ['C', 'D']

    # Test 3: Graph with loops
    graph3 = {'A': ['A', 'B'], 'B': ['A']}
    assert bfs_function(graph3, 'A') == ['A', 'B']

    # Test 4: Directed graph
    graph4 = {'A': ['B'], 'B': ['C'], 'C': []}
    assert bfs_function(graph4, 'A') == ['A', 'B', 'C']

    # Test 5: Graph with multiple possible BFS orders
    graph5 = {'A': ['B', 'C'], 'B': ['D'], 'C': ['D'], 'D': []}
    assert bfs_function(graph5, 'A') in [['A', 'B', 'C', 'D'], ['A', 'C', 'B', 'D']]

    print("All test cases pass")

def bfs_function(graph, start_node):
    visited = []
    queue = [start_node]

    while queue:
        v = queue.pop(0)
        if v not in visited:
            visited.append(v)
        for n in graph[v]:
            if n not in visited:
                queue.append(n)
    return visited








if __name__ == "__main__":
    test_breadth_first_search()
