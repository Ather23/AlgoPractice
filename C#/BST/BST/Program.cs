// See https://aka.ms/new-console-template for more information

using System.ComponentModel.DataAnnotations;

Console.WriteLine("Hello, World!");
var tree = new Tree<int>();

tree.Insert(100);
tree.Insert(30);
tree.Insert(200);

tree.PrintBreathFirst();



public class Node<T> where T: IComparable<T>
{
    public Node<T>? LeftNode { get;  set; }
    public Node<T>? RightNode { get; set; }

    public T Value { get; set; }

    public Node(T value)
    {
        Value = value;
    }
}



class Tree<T> : TreeOps<T> where T : IComparable<T>
{
    public Node<T>? Root { get; private set; }

    public void Insert(T? value)
    {        
        Root = InsertRecurse(value, Root);
    }

    private Node<T> InsertRecurse(T value, Node<T>? node)
    {
        if (node is null)
        {
            return new Node<T>(value);
        }

        var compareResult = value.CompareTo(node.Value);

        // val < node.val
        if (compareResult < 0)
        {
            node.LeftNode = InsertRecurse(value, node.LeftNode);
        }
        else if (compareResult > 0)
        {
            node.RightNode = InsertRecurse(value, node.RightNode);
        }
        
        return node;
    }


    public IEnumerable<Node<T>?> BreathFirst()
    {
        var visited= new HashSet<Node<T>>();
        var queue = new Queue<Node<T>?>();
        queue.Enqueue(Root);

        while (queue.Count != 0) {
            var node = queue.Dequeue();
            if ( node is not null)
            {
                visited.Add(node);
                queue.Enqueue(node.LeftNode);
                queue.Enqueue(node.RightNode);
            }
            yield return node;
        }
    }

    public void PrintBreathFirst()
    {
        var genNodes = BreathFirst();
        foreach(var node in genNodes)
        {
            if (node is not null)
            {
                Console.WriteLine(node.Value);
            }
        }
    }

    public void Search(T value)
    {

    }
}


interface TreeOps<T> where T: IComparable<T>
{
    void Insert(T? value);
    void Search(T? value);
    void PrintBreathFirst();

}

