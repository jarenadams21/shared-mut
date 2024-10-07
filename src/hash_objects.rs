/** NEEDS Hash and Peer sequences/cycles */

/**

Describes a latent byte encoded b-tree intended for mapping into q space
*/
struct ByteEncodedBTree {

}

struct ByteEncodedBNode {

}

struct ByteEncodedBLeaf {

}

/**

Describes a partial intermediate byte encoded b-tree intended for mapping into q space
*/
struct BytePartialEncodedBToQTree {

}

struct BytePartialEncodedBToQNode {

}

struct BytePartialEncodedBToQLeaf {

}

/**
Describes a custom byte encoded b-tree intended for q space
*/
struct ByteEncodedQBTree {

}

struct ByteEncodedQBNode {

}

struct ByteEncodedQBLeaf {

}

/**
 Holds data blocks containing information from classical web
*/
struct ClassicalWebDataSourceBlock {

}

/**
 Holds data blocks containing information relevant for the system before transforming into a q source block
*/
struct PartialSystemDataSourceBlock {

}

/** I think the idea from this point is classical web data sources that got scraped
    will process through the q system and become a QStateSourceBlock for search optimizations/processing.
 */

/**
 Holds data blocks containing information from q system state
*/
struct QStateSourceBlock {

}

struct QSpace {
    internal_time_index: Unknown,
}