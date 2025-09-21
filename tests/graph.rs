use signal_box_sim::track_graph::graph::TrackGraphBuilder;

#[test]
fn create_graph() {
    let mut graph = TrackGraphBuilder::new();
    graph.add_block("B v.u.n. A-Stadt".to_string(), "v.u.n. A-Stadt".to_string());
    graph.add_track(
        "v.u.n. A-Stadt".to_string(),
        "B v.u.n. A-Stadt".to_string(),
        "W1".to_string(),
    );
    graph.add_point(
        "W1".to_string(),
        "v.u.n. A-Stadt".to_string(),
        "1".to_string(),
        "2".to_string(),
    );
    graph.add_track(
        "1".to_string(),
        "W1".to_string(),
        "v.u.n. C-Dorf".to_string(),
    );
    graph.add_track("2".to_string(), "W1".to_string(), "2-E".to_string());
    graph.add_track(
        "v.u.n. C-Dorf".to_string(),
        "1".to_string(),
        "B v.u.n. C-Dorf".to_string(),
    );
    graph.add_block("B v.u.n. C-Dorf".to_string(), "v.u.n. C-Dorf".to_string());
    graph.add_dead_end("2-E".to_string(), "2".to_string());

    let json = graph.to_json_pretty().unwrap();

    assert_eq!(
        json,
        r#"[
  {
    "id": "B v.u.n. A-Stadt",
    "neighbours": {
      "Block": {
        "x": "v.u.n. A-Stadt"
      }
    }
  },
  {
    "id": "v.u.n. A-Stadt",
    "neighbours": {
      "Track": {
        "x": "B v.u.n. A-Stadt",
        "y": "W1"
      }
    }
  },
  {
    "id": "W1",
    "neighbours": {
      "Point": {
        "tip": "v.u.n. A-Stadt",
        "normal": "1",
        "reverse": "2",
        "coupled": null,
        "free_if_coupled_normal": null
      }
    }
  },
  {
    "id": "1",
    "neighbours": {
      "Track": {
        "x": "W1",
        "y": "v.u.n. C-Dorf"
      }
    }
  },
  {
    "id": "2",
    "neighbours": {
      "Track": {
        "x": "W1",
        "y": "2-E"
      }
    }
  },
  {
    "id": "v.u.n. C-Dorf",
    "neighbours": {
      "Track": {
        "x": "1",
        "y": "B v.u.n. C-Dorf"
      }
    }
  },
  {
    "id": "B v.u.n. C-Dorf",
    "neighbours": {
      "Block": {
        "x": "v.u.n. C-Dorf"
      }
    }
  },
  {
    "id": "2-E",
    "neighbours": {
      "DeadEnd": {
        "x": "2"
      }
    }
  }
]"#
        .to_string()
    );
}
